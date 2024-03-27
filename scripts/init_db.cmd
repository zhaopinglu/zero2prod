
sqlx database create -D postgres://postgres:postgres@localhost:5432/newsletter
sqlx migrate run -D postgres://postgres:postgres@localhost:5432/newsletter