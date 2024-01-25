GAUGE=3xC4eW6xhW3Gpb4T5sCKFe73ay2K4aUUfxL57XFdguJx
EPOCH=96
# Change this escrow if you generate localhost accounts with another key
ESCROW=FXLMDGpLirbot2DY9JCmeKZtvu1TFaWSszE1Bd1kQAn
pkill -f solana-test-validator
anchor localnet > /dev/null 2>&1 &
sleep 5
MINT=$(cargo run --quiet -- create-token 2>/dev/null | tail -n 1 | awk '{print $2}')
echo The mint is $MINT
CONFIG=$(cargo run --quiet -- setup --mints $MINT 2>/dev/null | tail -n 1 | awk '{print $2}')
echo The config is $CONFIG
cargo run -- buy-votes $CONFIG $GAUGE $MINT 100 2>/dev/null
# Vote
cargo run -- delegate $ESCROW $CONFIG 2> /dev/null
cargo run -- vote $ESCROW $CONFIG $EPOCH 2>/dev/null

# Redeem