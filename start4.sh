# !/bin/bash

rm -rf /tmp/val4

./insertKey.sh "//Dave" "/tmp/val4"

address=68.79.37.147

#./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val2 --name=myNode2 --validator --port=30334 --ws-port=9945 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWSMNLnMk1CpwEn8N8UHe7dnHyYYYLUqiSioQv1f5xkeN5 --node-key=a31990869536943efb5de647e38cece4bfe55d072b76b4a032771b96aa5ec9f1

./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val4 --name=myNode4 --execution=Native  --validator --port=30336 --ws-port=9947 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --rpc-methods=unsafe --bootnodes=/ip4/$address/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 --node-key=a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a
