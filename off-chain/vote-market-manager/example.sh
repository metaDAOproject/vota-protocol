MINT=$(cargo run -- create-token | tail -n 1 | awk '{print $2}')
echo The mint is $MINT
CONFIG=$(cargo run -- setup --mints $MINT | tail -n 1 | awk '{print $2}')
echo The config is $CONFIG