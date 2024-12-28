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
# CONTAINER="rapidfort/postgresql"
CONTAINER="postgres"
CONTAINER_NAME="postgres"

# Check if the container exists and its status
if docker ps -a -f name="${CONTAINER_NAME}" --format '{{.ID}}' | grep -q .; then
    # Container exists, check if it's running
    if docker ps -f name="${CONTAINER_NAME}" --format '{{.ID}}' | grep -q .; then
        echo "Container ${CONTAINER_NAME} is already running."
    else
        echo "Container ${CONTAINER_NAME} exists but is not running. Starting..."
        docker start "${CONTAINER_NAME}"
    fi
else
    # Create a temporary directory for initialization scripts
    INIT_DIR=$(mktemp -d)
    cp scripts/init.sql "${INIT_DIR}/"

    # Replace variables in the init.sql file
    sed -i "s/\${APP_USER}/${APP_USER}/g" "${INIT_DIR}/init.sql"
    sed -i "s/\${APP_USER_PWD}/${APP_USER_PWD}/g" "${INIT_DIR}/init.sql"
    sed -i "s/\${APP_DB_NAME}/${APP_DB_NAME}/g" "${INIT_DIR}/init.sql"

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
        --volume "${INIT_DIR}/init.sql":/docker-entrypoint-initdb.d/init.sql \
        --detach \
        ${CONTAINER}

    # Clean up the temporary directory
    rm -rf "${INIT_DIR}"
fi

# Wait for Postgres to be ready
until [ \
    "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
    "healthy" \
]; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

sleep 1

sqlx migrate run
>&2 echo "Postgres has been migrated, ready to go!"
