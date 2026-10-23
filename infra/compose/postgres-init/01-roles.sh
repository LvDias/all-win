#!/bin/sh
# Runs once on first start of an empty data directory (docker-entrypoint-initdb.d), as the superuser.
# Creates the Core's login roles and the `allwin` database (owner allwin_owner). Schema, grants and RLS
# policies belong to the Core migrations (applied by the binary via MIGRATION_DATABASE_URL).
set -eu
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname postgres \
  -v owner_pw="$ALLWIN_OWNER_PASSWORD" -v app_pw="$ALLWIN_APP_PASSWORD" \
  -v worker_pw="$ALLWIN_WORKER_PASSWORD" -v maint_pw="$ALLWIN_MAINT_PASSWORD" \
  -v publisher_pw="$ALLWIN_PUBLISHER_PASSWORD" <<'SQL'
CREATE ROLE allwin_owner  LOGIN NOBYPASSRLS PASSWORD :'owner_pw';
CREATE ROLE allwin_app    LOGIN NOBYPASSRLS PASSWORD :'app_pw';
CREATE ROLE allwin_worker LOGIN BYPASSRLS   PASSWORD :'worker_pw';
CREATE ROLE allwin_maint  LOGIN BYPASSRLS   PASSWORD :'maint_pw';
-- Used only by the publish-release CLI (maintenance path, S16). No BYPASSRLS; grants come from migrations.
CREATE ROLE allwin_publisher LOGIN NOBYPASSRLS PASSWORD :'publisher_pw';
CREATE DATABASE allwin OWNER allwin_owner;
REVOKE ALL ON DATABASE allwin FROM PUBLIC;
GRANT CONNECT ON DATABASE allwin TO allwin_app, allwin_worker, allwin_maint, allwin_publisher;
SQL
