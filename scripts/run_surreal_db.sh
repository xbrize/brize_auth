#!/bin/bash
docker run --name test_db -d --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start;
cargo run;
docker kill test_db;