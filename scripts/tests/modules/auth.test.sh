#!/bin/bash
docker run --name sql_db -d --rm --pull always -p 3306:3306 -e MYSQL_ROOT_PASSWORD=my-secret-pw mysql:latest;
docker run --name surreal_db -d --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start --user root --password surreal_ps;
sleep 10;
cargo test auth;
docker kill surreal_db;
docker kill sql_db;