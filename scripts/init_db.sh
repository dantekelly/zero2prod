#!/usr/bin/env bash
set -x
set -eo pipefail

# Ensure Sqlx CLI is installed
if ! command -v sqlx &> /dev/null; then
    echo "Sqlx CLI could not be found. Installing..."
    cargo install sqlx-cli --no-default-features --features rustls,postgres
fi

# Check if a custom parameter has been set, otherwise use default values
DB_PORT="${POSTGRES_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"

APP_USER="${APP_USER:=app}"
APP_USER_PWD="${APP_USER_PWD:=secret}"
APP_DB_NAME="${APP_DB_NAME:=newsletter}"


# Launch postgres using Docker
CONTAINER="rapidfort/postgresql"
CONTAINER_NAME="postgres"

# TODO: Change this to a check for the container already existing, atm if the container exists and is not running, it will fail.
# Check if the container is already running
if docker ps -f name="${CONTAINER_NAME}" --format '{{.ID}}' | grep -q .; then
    echo "Container ${CONTAINER_NAME} is already running."
else
    docker run \
        --env POSTGRES_USER=${SUPERUSER} \
    --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
    --env POSTGRESQL_DATABASE=${APP_DB_NAME} \
    --name "${CONTAINER_NAME}" \
    --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
    --health-interval=1s \
    --health-timeout=5s \
    --health-retries=5 \
    --publish "${DB_PORT}":5432 \
        --detach \
        ${CONTAINER}
fi

# Wait for Postgres to be ready to accept connections
until [ \
    "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
    "healthy" \
]; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

# Create the application user
CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"
# Grant create db privileges to the app user
GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
