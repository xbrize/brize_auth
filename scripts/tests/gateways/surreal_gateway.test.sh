#!/bin/bash
docker run --name surreal_db -d --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start;
sleep 5;
cargo test surreal;
docker kill surreal_db;