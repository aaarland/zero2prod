#!/usr/bin/env bash
set -x
set -eo pipefail

if ! command -v psql; then
  echo >&2 'Error: psql is not installed.'
  exit 1
fi

if ! command -v sqlx ;then
  echo >&2 'Error: sqlx is not installed.'
  echo >&2 'Install it with: cargo install sqlx-cli --version=0.5.7 --no-default-features --features postgres'
  exit 1
fi

# Check if custom user had been set otherwise default to postgres
DB_USER="${DB_USER:=postgres}"
# Check if custom password had been set otherwise default to postgres
DB_PASS="${DB_PASS:=postgres}"
# Check if custom database name had been set otherwise default to newsletter
DB_NAME="${DB_NAME:=newsletter}"
# Check if a custom port has been set otherwise default to 5432
DB_PORT="${DB_PORT:=5432}"

if [[ -z "${SKIP_DOCKER}" ]]; then
# Launch postgres using Docker
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASS} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    #Increased maximum number of connections for testing purposes
fi




# Keep pinging Postgres until it's ready to accept connections
export PGPASSWORD="${DB_PASS}"
until psql -h localhost -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done
export DATABASE_URL="postgresql://${DB_USER}:${DB_PASS}@127.0.0.1:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
