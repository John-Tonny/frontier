# !/bin/bash

rm -rf /tmp/bob

./insertKey.sh "//Bob" "/tmp/bob"

#./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val2 --name=myNode2 --validator --port=30334 --ws-port=9945 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWSMNLnMk1CpwEn8N8UHe7dnHyYYYLUqiSioQv1f5xkeN5 --node-key=a31990869536943efb5de647e38cece4bfe55d072b76b4a032771b96aa5ec9f1

./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/bob --name=myNode2 --validator --port=30334 --ws-port=9945 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --rpc-methods=unsafe --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 --node-key=6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58
