# Solana Transfer Extraction (Substreams)

Substreams module that extracts transfers from Solana blocks 385870151-385870156.
It covers native SOL transfers (System Program) and SPL token transfers
(Token and Token-2022: Transfer and TransferChecked), including inner instructions.

Output per transfer: {"from": "...", "to": "...", "amount": 123}

## Build
substreams build

## Run
export SUBSTREAMS_API_KEY=<your key>
substreams run -e solana.substreams.pinax.network:443 solana-transfers-v0.1.0.spkg map_transfers \
  -s 385870151 -t 385870157 -o jsonl

`output.jsonl` contains the result for the required block range (3974 transfers).
