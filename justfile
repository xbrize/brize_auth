publish-changelog version:
    git cliff -u --tag {{version}} --prepend CHANGELOG.md

test-mysql:
    -scripts/tests/gateways/mysql_gateway.test.sh

test-surreal:
    -scripts/tests/gateways/surreal_gateway.test.sh

test-domain:
    cargo test domain

test-application:
    ./scripts/init_db.sh
    -cargo test application
    ./scripts/stop_db.sh

test-infra:
    ./scripts/init_db.sh
    -cargo test infrastructure
    ./scripts/stop_db.sh

migrate-add file_name:
    @source .env \
    && sqlx migrate add --source database/migrations {{file_name}}