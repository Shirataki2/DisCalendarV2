if [ $1 = '--dev' ]; then
bash
else
yarn build
yarn start
fi
