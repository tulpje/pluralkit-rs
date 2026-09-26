#!/usr/bin/env bash
set -u

# set env vars
export POSTGRES_USER="pluralkit"
export POSTGRES_PASSWORD="pluralkit"
export POSTGRES_DB="pluralkit"

export MARIADB_USER="pluralkit"
export MARIADB_PASSWORD="pluralkit"
export MARIADB_DATABASE="pluralkit"

# start containers and wait for ready
docker compose -f compose.test.yaml up --wait --detach

# configure database urls
export MYSQL_IP=$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' pluralkit-rs-mariadb-1)
export MYSQL_URL=mysql://${MARIADB_USER}:${MARIADB_PASSWORD}@${MYSQL_IP}/${MARIADB_DATABASE}

export POSTGRES_IP=$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' pluralkit-rs-postgres-1)
export POSTGRES_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_IP}/${POSTGRES_DB}

# run tests
cargo test --all-features -- --include-ignored

# cleanup
docker compose -f compose.test.yaml down -v
