# !/bin/bash

rm -rf /tmp/val1

#nodekey=8073abf88f075680719a9ac1e04aaef643cf40f8e3282bdf82ff66962f0e054e
nodekey=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a

#./insertKey.sh "arrow veteran arm circle gentle modify solar speak tumble alien finish caution//liuFirst" "/tmp/val1"
./insertKey.sh "//Alice" "/tmp/val1"

./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val1 --sync=Full --name=myNode1 --validator --node-key=$nodekey --port=30333 --ws-port=9944 --unsafe-ws-external --unsafe-rpc-external --rpc-methods=unsafe --rpc-cors=all

#./target/debug/frontier-template-node --chain=./customSpecRaw.json --sync=full --base-path=/tmp/val1 --name=myNode1 --execution=Native  --validator --node-key=c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a --port=30333 --rpc-port=9933 --ws-port=9944 --unsafe-ws-external --unsafe-rpc-external --rpc-methods=unsafe --rpc-cors=all
