#!/bin/bash

echo $KEY > /usr/src/vmm/keypair.json
echo $KEY2 > /usr/src/vmm/keypair2.json

exec /usr/src/vmm/target/release/vote-market-manager get-vote-buys F72CPZ7vumQ6Z7e5ncWxkNunzcL79xkjTaiNCvZoL7Uc 109