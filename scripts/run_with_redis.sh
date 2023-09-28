#!/bin/bash
docker run --name redis_db -d --rm --pull always -p 6379:6379 redis redis-server --requirepass mypassword
cargo run;
docker kill redis_db;

