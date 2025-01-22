const players = []
const edges = []
const playersMap = new Map()

const graph = {
    players,
    edges,
}

const matchesList = document.querySelectorAll('.brkts-matchlist')
matchesList.forEach(table => {
    const lines = table.querySelectorAll('.brkts-matchlist-match')
    lines.forEach(line => {
        const [first, second] = Array.from(line.querySelectorAll('.brkts-matchlist-cell .name'))
            .map(span => span.innerHTML)
            .map(playerName => {
                // Checks if the name already exists in the map
                const fromMap = playersMap.get(playerName.toLowerCase())
                if (fromMap) return fromMap
                
                // Otherwise, create a new player
                const player = {
                    name: playerName,
                    id: players.length,
                }
                players.push(player)
                playersMap.set(playerName.toLowerCase(), player)

                return player
            })

        const [firstWins, secondWins] = Array.from(line.querySelectorAll('.brkts-matchlist-score .brkts-matchlist-cell-content'))
            .map(div => Number.parseInt(div.innerHTML))

        // Adds edges
        for (let i = 0; i < firstWins; i++) {
            edges.push({
                winner_id: first.id,
                loser_id: second.id,
            })
        }

        for (let i = 0; i < secondWins; i++) {
            edges.push({
                winner_id: second.id,
                loser_id: first.id,
            })
        }
    })
})

console.log(`Registered ${players.length} players and ${edges.length} matches`)

console.log(JSON.stringify(graph))