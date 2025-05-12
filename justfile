dev:
  find src | entr -r cargo run

reset-db:
  rm -f database.db
  cat schema.sql | sqlite3 database.db
  python import.py
