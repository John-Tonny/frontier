# !/bin/bash

rm -rf /tmp/val1

./insertKey.sh "//Alice" "/tmp/val1"

#./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val1 --name=myNode1 --validator --node-key=fa367802c8a68c3d9c6307b121e43e028c9dbfa5cb92ba37bc8ad6e1c29d8c6c --port=30333 --ws-port=9944 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all

./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val1 --name=myNode1 --execution=Native  --validator --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a --port=30333 --rpc-port=9933 --ws-port=9944 --unsafe-ws-external --unsafe-rpc-external --rpc-methods=unsafe --rpc-cors=all
