test-redis:
    -scripts/tests/gateways/redis_gateway.test.sh

test-mysql:
    -scripts/tests/gateways/mysql_gateway.test.sh

test-surreal:
    -scripts/tests/gateways/surreal_gateway.test.sh

publish-changelog:
    git cliff bd18f09.. --tag 0.8.0 --prepend CHANGELOG.md    