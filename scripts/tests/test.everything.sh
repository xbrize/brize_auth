#!/bin/bash
docker run --name surreal_db -d --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start;
docker run --name redis_db -d --rm --pull always -p 6379:6379 redis redis-server --requirepass mypassword
cargo test;
docker kill surreal_db;
docker kill redis_db;