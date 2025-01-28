#!/bin/bash
source .env
docker run --name mysql_db -d --rm -p "${DB_PORT}":"${DB_PORT}" -e MYSQL_ROOT_PASSWORD=${DB_PASSWORD} mysql:latest;

# Keep pinging MySQL until it's ready to accept commands
until mysql -h 127.0.0.1 -u "${DB_USER}" -p"${DB_PASSWORD}" -P "${DB_PORT}" -D "${DB_NAME}" -e 'SELECT 1'; do
  >&2 echo "MySQL is still unavailable - sleeping"
  sleep 1
done

# create migration with sqlx
sqlx database create
sqlx migrate run --source database/migrations