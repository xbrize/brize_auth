#!/bin/bash
docker run --name my_sql_db -d --rm --pull always -p 3306:3306 -e MYSQL_ROOT_PASSWORD=my-secret-pw mysql:latest;
sleep 5;
cargo test MySql;
docker kill my_sql_db;