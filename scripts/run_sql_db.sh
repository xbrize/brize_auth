#!/bin/bash
docker run --name test_db -d --rm --pull always -p 3306:3306 -e MYSQL_ROOT_PASSWORD=my-secret-pw mysql:latest;
cargo run;
docker kill test_db;
