publish-changelog version:
    git cliff -u --tag {{version}} --prepend CHANGELOG.md

migrate-add file_name:
    @source .env \
    && sqlx migrate add --source database/migrations {{file_name}}

# Tests
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

test-all:
    ./scripts/init_db.sh
    -cargo test
    ./scripts/stop_db.sh

test-mysql:
    ./scripts/init_db.sh
    cargo test mysql
    ./scripts/stop_db.sh
