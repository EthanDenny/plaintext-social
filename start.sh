if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
fi

docker compose down
docker compose up -d

cargo run -p migration -- fresh

sea-orm-cli generate entity \
  -u $DATABASE_URL \
  -o src/entities \

cargo +nightly test

watchexec -e rs,html,js,css -r cargo +nightly run
