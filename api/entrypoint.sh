if [ $1 = '--dev' ]; then
bash
else
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
cargo install --path .
serve
fi
