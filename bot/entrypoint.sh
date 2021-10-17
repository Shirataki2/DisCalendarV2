if [ $1 = '--dev' ]; then
bash
else
cargo install --path .
bot
fi
