rem #curl -i -X POST -d "email=aaa5@bbb.com&username=lzp" http://127.0.0.1:8000/subscriptions
rem #curl -v http://127.0.0.1:8000/health_check


set RUST_LOG="sqlx=error,info"
set TEST_LOG=true
rem cargo t subscribe_fails_if_there_is_a_fatal_database_error | bunyan
cargo t subscriptions::subscribe_returns_a_400_when_data_is_missing | bunyan
