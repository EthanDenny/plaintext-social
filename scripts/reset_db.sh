if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
fi

cargo run -p migration -- fresh

sea-orm-cli generate entity \
  -u $DATABASE_URL \
  -o src/entities \
