[features]
seeds = false
[programs.localnet]
meme_collector = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"

[registry]
url = "https://anchor.projectserum.com"

[provider]
cluster = "localnet"
wallet = "/Users/nu9ve/.config/solana/id.json"

[scripts]
test = "node tests/m-collector.js"
test = "yarn run mocha -t 1000000 tests/"
