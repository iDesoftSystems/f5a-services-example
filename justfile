start:
    cargo run

check:
    cargo clippy --all-targets --all-features -- -D warnings

docker-compose:
    docker compose up --build -d --remove-orphans

docker-start:
    docker compose start

docker-stop:
    docker compose stop

migrate-fresh:
    sea-orm-cli migrate fresh

gen-entity:
    sea-orm-cli generate entity -o ./schemas/src --lib
