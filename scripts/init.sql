-- Setup User
CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';
ALTER USER ${APP_USER} CREATEDB;

-- Create newsletter database
CREATE DATABASE ${APP_DB_NAME};
ALTER DATABASE ${APP_DB_NAME} OWNER TO ${APP_USER};

-- Connect to the newsletter database and grant schema privileges
\c ${APP_DB_NAME}
GRANT ALL ON SCHEMA public TO ${APP_USER};
