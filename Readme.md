# Pokespeare - A simple CRUD app written in Rust

![Build passing](https://github.com/nindalf/pokespeare/actions/workflows/rust.yml/badge.svg) [![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/nindalf/pokespeare)](https://rust-reportcard.xuri.me/report/github.com/nindalf/pokespeare) ![License](https://img.shields.io/github/license/nindalf/pokespeare)

## Problem statement

> Given a Pokemon name, return it's Shakespearean description.

Example

```bash

➜ curl http://localhost:8000/pokemon/charizard

{
    "name": "charizard",
    "description": "Charizard flies 'round the sky in search of powerful opponents. 't breathes fire of such most wondrous heat yond 't melts aught. However, 't nev'r turns its fiery breath on any opponent weaker than itself."
}
```

Useful APIs

- [Shakespeare translator](https://funtranslations.com/api/shakespeare)
- [PokéAPI](https://pokeapi.co)

---

## Setup

Install Rust ([instructions](https://www.rust-lang.org/tools/install)) and Docker ([instructions](https://docs.docker.com/get-docker/))

Step 1 - Initial setup - clone the repo and set up the database

```bash
➜ git clone https://github.com/nindalf/pokespeare.git

➜ cd pokespeare

➜ ./scripts/init_db.sh
# + set -eo pipefail
# ...
# Postgres has been migrated, ready to go!
```

Step 2 - Run the app
```bash
➜ cargo run
#    Compiling pokespeare v0.1.0 (/Users/nindalf/Repos/pokespeare)
#     Finished dev [unoptimized + debuginfo] target(s) in 7.19s
#      Running `target/debug/pokespeare`

# Make a request
➜ curl localhost:8000/pokemon/pikachu
# {"name":"pikachu","description":"At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms."}

```

Step 3 - Run unit tests and integration tests

```bash
➜ cargo test
# ...
# running 9 tests
# test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s
# ...
# running 2 tests
# test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.26s

```

To customize the configuration, change `configuration.yaml`.

---

## Improvements

Areas where this app should improve, in order of urgency+importance. 

1. Add structured logging and sink for logs
2. Add monitoring - dashboards and alerts.
3. Improve deployment story. Continuous deployment. 
4. Deploy to multiple nodes with loadbalancing.
5. Database sharding/replication.
6. Auto-scaling.
