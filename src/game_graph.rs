use std::{collections::HashMap, rc::Rc, sync::RwLock};

use serde_json::json;
use thiserror::Error;

use crate::{
    graph::{Graph, GraphEdge, GraphNode},
    json_data::{JsonGraph, JsonPlayer},
    rating_system,
};

/// Represents a player in the game graph.
#[derive(Debug)]
pub struct Player {
    /// The player name
    pub name: String,
    /// The player rating
    /// This rating scale is different from the Elo scale. Here, the base rating is 0 and the expected outcome of a
    /// match is given by 1 / (1 + (b-a).exp()) to make the calculations simpler and faster and I use to call it LSR
    /// (Lanna Rating System). LSR scale can be easily converted into Elo rating scale by multiplying by 400 * log10(e).
    pub rating: f64,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name, rating: 0.0 }
    }
}

/// Represents the possible outcomes of a match.
#[derive(Debug)]
pub enum MatchResult {
    Victory,
    Defeat,
}

pub type GameNode = GraphNode<Player, MatchResult>;
pub type GameEdge = GraphEdge<Player, MatchResult>;
pub type GameGraph = Graph<Player, MatchResult>;

impl GameNode {
    /// Gets the likelihood that the player has that score, given it's matches. This does not consider any priori
    /// distribution.
    pub fn likelihood(&self, score: f64) -> f64 {
        let mut likelihood = 1.0;
        for edge in self.iter_edges() {
            let lock = edge.pointer.read().expect("failed to get read lock");
            if let MatchResult::Victory = edge.data {
                likelihood *= rating_system::odds(score, lock.rating);
            } else {
                likelihood *= rating_system::odds(lock.rating, score);
            }
        }

        likelihood
    }

    /// Gets how many games the player won.
    pub fn wins(&self) -> usize {
        let mut count = 0;
        self.iter_edges().for_each(|edge| {
            if let MatchResult::Victory = edge.data {
                count += 1;
            }
        });
        count
    }

    /// Gets how many games the player lost.
    pub fn defeats(&self) -> usize {
        let mut count = 0;
        self.iter_edges().for_each(|edge| {
            if let MatchResult::Defeat = edge.data {
                count += 1;
            }
        });
        count
    }
}

impl GameGraph {
    /// Adds a match between two players to the graph.
    pub fn add_match(winner: Rc<RwLock<GameNode>>, loser: Rc<RwLock<GameNode>>) {
        let winner_edge = GameEdge::new(MatchResult::Victory, loser.clone());
        let loser_edge = GameEdge::new(MatchResult::Defeat, winner.clone());
        let mut winner = winner.write().expect("failed to get lock");
        let mut loser = loser.write().expect("failed to get lock");
        winner.add_edge(winner_edge);
        loser.add_edge(loser_edge);
    }

    /// Exports the graph to a JSON containing the rating of each player, but not it's matches.
    pub fn export_json(&self) -> serde_json::Value {
        let players: Vec<_> = self
            .iter_nodes()
            .map(|player| {
                let player = player.read().expect("failed to get read lock");
                json!({
                    "player": player.name,
                    "rating": rating_system::convert_to_elo(player.rating),
                    "wins": player.wins(),
                    "defeats": player.defeats(),
                })
            })
            .collect();

        json!(players)
    }
}

#[derive(Error, Debug)]
pub enum GraphParseError {
    #[error("the json graph contains two player with the same id: {0}")]
    RedundantId(u8),
    #[error("a match has pointed to an invalid player id: {0}")]
    InvalidId(u8),
}

impl TryFrom<JsonGraph> for GameGraph {
    type Error = GraphParseError;

    fn try_from(json_graph: JsonGraph) -> Result<Self, Self::Error> {
        eprintln!("converting json graph into game graph...");
        let JsonGraph {
            players: json_players,
            edges: json_edges,
        } = json_graph;

        let mut game_graph = GameGraph::new();

        // First, populate the a hashmap that maps player ids to players.
        eprintln!("found {} players. extracting...", json_players.len());
        let mut player_map = HashMap::with_capacity(json_players.len());
        for json_player in json_players {
            // Destructure json_player so we can move name and id into separate places.
            let JsonPlayer { name, id } = json_player;

            // Adds the player to the graph.
            eprintln!("adding {name}");
            let graph_player = game_graph.add_node(Player::new(name));

            // Adds the player to the hash map.
            if let Some(_) = player_map.insert(id, graph_player) {
                // If there was anything before, raise an error instead of continuing.
                return Err(GraphParseError::RedundantId(id));
            }
        }
        eprintln!("done extracting players.");

        // Populates the graph with the matches pointed in the json graph.
        eprintln!("found {} matches. extracting...", json_edges.len());
        for json_result in json_edges {
            // Gets the players from the hashmap. If one was not found, return an error.
            let winner = player_map
                .get(&json_result.winner_id)
                .map(Clone::clone)
                .ok_or(GraphParseError::InvalidId(json_result.winner_id))?;
            let loser = player_map
                .get(&json_result.loser_id)
                .map(Clone::clone)
                .ok_or(GraphParseError::InvalidId(json_result.loser_id))?;

            GameGraph::add_match(winner, loser);
        }
        eprintln!("done extracting matches.");

        eprintln!("done converting json graph into game graph");
        Ok(game_graph)
    }
}
