#!/bin/bash
docker run --name surreal_db -d --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start;
cargo test surreal_credentials_repo;
cargo test surreal_session_repo;
docker kill surreal_db;