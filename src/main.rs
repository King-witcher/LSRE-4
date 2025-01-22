/// Module for handling calculus and integrals.
mod calculus;
/// Module for building game graphs, based on `graph`.
mod game_graph;
/// Module for building graphs.
mod graph;
/// Deserialized json data.
mod json_data;
/// Module for handling the rating system logic.
mod rating_system;

use anyhow::Result;
use clap::{command, Parser};
use game_graph::GameGraph;
use json_data::JsonGraph;
use rating_system::LSR_TO_ELO_RATIO;
use std::{fs::File, io::Write, path::Path};

/// A Rust tool for estimating player Elo ratings based on a match graph through Bayes Inference.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// The path to the LSRE Json Graph
    path: String,

    /// The output file. If this is not defined, LSRE prints to the standard output.
    #[arg(short, long)]
    output: Option<String>,

    /// The number of estimation rounds to run.
    #[arg(short = 'n', long, default_value_t = 6)]
    count: u8,

    /// The infinitesimal dx value.
    #[arg(short, long, default_value_t = 1e-5)]
    dx: f64,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut graph = extract_json_graph(args.path)?;

    estimate(&mut graph, args.count, args.dx);
    let json = graph.export_json();

    eprintln!("done!");

    match args.output {
        Some(output) => {
            let mut file = File::create(output)?;
            write!(file, "{json:#}")?;
        }
        None => {
            println!("{json:#}");
        }
    }
    Ok(())
}

/// Extracts the json graph from a file.
fn extract_json_graph<T: AsRef<Path>>(path: T) -> Result<GameGraph> {
    eprintln!("extracting json graph...");
    let file = File::open(path)?;
    let json_graph = serde_json::from_reader::<File, JsonGraph>(file)?;
    GameGraph::try_from(json_graph).map_err(From::from)
}

/// The priori rating distribution. This considers that the initial distribution has a standard deviation of 200 Elo
/// points.
fn priori(x: f64) -> f64 {
    const SIGMA: f64 = 200.0 / LSR_TO_ELO_RATIO;

    (-x.powi(2) / (2.0 * SIGMA)).exp()
}

/// Estimates the rating of the players in a graph in rounds. Each round, estimate the rating of each player via Bayes
/// Inference, considering the priori function defined above, after the set of events (match outcomes) that each player
/// had, and the rating of whom he has played against.
///
/// Each round first estimates the rating of each individual player considering the previous rating of each one and,
/// after that, updates the rating of every one.
///
/// On each round, prints to the standard error output the displacement between the current and previous state. The
/// displacement is calculated based on the "distance" between each one, considering them as vectors of Elo scores.
fn estimate(graph: &mut GameGraph, rounds: u8, dx: f64) {
    for round in 0..rounds {
        eprint!("round {}/{rounds}... ", round + 1);
        let mut square_displacements = 0.0;
        let mut player_count = 0.0;

        graph
            .iter_nodes()
            // Estimate a new rating for each player.
            .map(|player| {
                let read_lock = player.read().unwrap();
                let expected_lsr = calculus::avg_value(|x| priori(x) * read_lock.likelihood(x), dx);

                // Computes the current displacement.
                square_displacements += (rating_system::convert_to_elo(read_lock.rating)
                    - rating_system::convert_to_elo(expected_lsr))
                .powi(2);
                player_count += 1.0;

                (player.clone(), expected_lsr)
            })
            // Store the new ratings each player must have before actually updating them.
            .collect::<Vec<_>>()
            .into_iter()
            // Finally, update the rating of every player simultaneously.
            .for_each(|(player, estimated_score)| {
                let mut player = player.write().expect("failed to get write lock");
                player.rating = estimated_score;
            });

        let avg_displacement = (square_displacements / player_count).sqrt();
        eprintln!("displacement: {avg_displacement}");
    }
}
