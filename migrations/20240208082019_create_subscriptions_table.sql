-- ref: https://www.postgresql.org/download/linux/redhat/
-- install pg16 on 192.168.55.200:
-- sudo dnf install -y https://download.postgresql.org/pub/repos/yum/reporpms/EL-8-x86_64/pgdg-redhat-repo-latest.noarch.rpm
-- sudo dnf -qy module disable postgresql
-- sudo dnf install -y postgresql16-server
-- sudo /usr/pgsql-16/bin/postgresql-16-setup initdb
-- sudo systemctl enable postgresql-16
-- sudo systemctl start postgresql-16


-- ref: zero2prod p80
-- Install sqlx:
-- cargo install sqlx-cli

-- ref: zero2prod p81
-- Create db
-- set DATABASE_URL=postgres://postgres:MyNewPass4!@192.168.55.200:5432/newsletter
-- sqlx database create
-- sqlx migrate run

-- Config .env file:
-- DATABASE_URL="postgres://postgres:MyNewPass4!@192.168.55.200:5432/newsletter"

-- Subscriptions
-- curl -i -X POST -d "email=aaa@bbb.com&username=lzp" http://127.0.0.1:8000/subscriptions


-- Sqlx Offline Mode
-- cargo clean
-- cargo sqlx prepare --workspace

-- Add migration script here
-- migrations/{timestamp}_create_subscriptions_table.sql
-- Create Subscriptions Table
CREATE TABLE subscriptions
(
    id            uuid        NOT NULL,
    PRIMARY KEY (id),
    email         TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at timestamptz NOT NULL
);