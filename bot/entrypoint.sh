if [ $1 = '--dev' ]; then
bash
else
apt update -y
apt install -y opus-tools
cargo install --path .
bot
fi
