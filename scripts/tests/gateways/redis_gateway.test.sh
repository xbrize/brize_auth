#!/bin/bash
docker run --name redis_db -d --rm --pull always -p 6379:6379 redis redis-server --requirepass mypassword;
sleep 5;
cargo test redis;
docker kill redis_db;