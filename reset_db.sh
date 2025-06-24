export DATABASE_URL=mysql://root:password@localhost:3306/plaintext_social

cargo run -p migration -- fresh --database-url $DATABASE_URL

sea-orm-cli generate entity \
  -u $DATABASE_URL \
  -o src/entities \

cargo +nightly test
