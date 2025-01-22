# How to use

## Extracting graphs

First, you will need a *LSRE JSON Graph*, which represents both the player names and the set of matches and the matches that you want to analyze.

A LSRE JSON Graph comes in the format of:

```ts
type GraphNode = {
    name: string
    id: number
}

type GraphEdge = {
    winner_id: number
    loser_id: number
}

/// The LSRE JSON Graph type
type Graph = {
    players: GraphNode[],
    edges: GraphEdge[]
}
```

Great examples of graphs can be found in the `graphs` directory, based on all Quake Pro League seasons.

The easiest way to get graphs is by scraping data from websites using predefined scripts, such as scripts found in the `scripts` directory, made for Liquipedia.

## Using a graph

Once you got a graph ready, just LSRE it with:

`cargo run --release graphs/my-graph.json`

And LSRE will spit out a JSON with the info you need.