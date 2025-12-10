gen-users:
    uv run python ./data/processed/users.py ./data/raw/users.csv

start-neo4j:
    docker-compose up

rm-neo4j:
    docker-compose down -v

backend-watch:
    cd ./backend && \
    watchexec -r -w src -w Cargo.toml \
        RUST_LOG=trace,sled=warn,facet_json=warn cargo run --profile act -- \
        --port 6232 \
        --address '::' \
        --neo-uri 'bolt://127.0.0.1:7687' \
        --neo-username 'neo4j' \
        --neo-password '1234567890'

