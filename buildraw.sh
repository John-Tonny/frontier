# !/bin/bash

#chain=customSpec.json
chain=testnet

./target/debug/frontier-template-node build-spec --chain=${chain}.json --raw --disable-default-bootnode > ${chain}Raw.json
