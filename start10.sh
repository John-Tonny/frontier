# !/bin/bash

rm -rf /tmp/val10

./insertKey.sh "//Bbb" "/tmp/val10"

#./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val2 --name=myNode2 --validator --port=30334 --ws-port=9945 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWSMNLnMk1CpwEn8N8UHe7dnHyYYYLUqiSioQv1f5xkeN5 --node-key=a31990869536943efb5de647e38cece4bfe55d072b76b4a032771b96aa5ec9f1

./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val10 --name=myNode10 --execution=Native  --validator --port=31333 --ws-port=19944 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --rpc-methods=unsafe --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 --node-key=669a0c6c20a914f9bd2b015d65e527f096e2b58415c182ac6a2dab0769b33952
