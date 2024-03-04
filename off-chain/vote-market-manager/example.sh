GAUGE=3xC4eW6xhW3Gpb4T5sCKFe73ay2K4aUUfxL57XFdguJx
EPOCH=94
# Change this escrow if you generate localhost accounts with another key
ESCROW=FXLMDGpLirbot2DY9JCmeKZtvu1TFaWSszE1Bd1kQAn
OWNER=8Df9mQfYfVj3uMjdhxMfF41PwbxC5xZofsHHdgyvG5Gr
pkill -f solana-test-validator
anchor localnet > /dev/null 2>&1 &
sleep 5
MINT=$(cargo run --quiet -- create-token 2>/dev/null | tail -n 1 | awk '{print $2}')
echo The mint is $MINT
CONFIG=$(cargo run --quiet -- setup --mints $MINT 2>/dev/null | tail -n 1 | awk '{print $2}')
echo The config is $CONFIG
cargo run -- buy-votes $CONFIG $GAUGE $MINT $EPOCH 5555555
## Vote
DELEGATE=$(cargo run -- delegate $CONFIG | tail -n 1 | awk '{print $2}')
echo The delegate is $DELEGATE
solana transfer $DELEGATE 0.1 --allow-unfunded-recipient
cargo run -- vote-test $OWNER $CONFIG $EPOCH
##
cargo run -- trigger-epoch 2> /dev/null
cargo run -- set-maximum $CONFIG $GAUGE 5555555 $EPOCH
cargo run -- claim $MINT $ESCROW $CONFIG $GAUGE $EPOCH
