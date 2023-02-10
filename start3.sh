# !/bin/bash


nodekey=54577e3aeec0501b550cad7ba0fc7dc3f501f698d5043b547f2ddbf23b8e8c70

rm -rf /tmp/val3/chains

./insertKey.sh "arrow veteran arm circle gentle modify solar speak tumble alien finish caution//zhangThird" "/tmp/val3"
#./insertKey.sh "//Charlie" "/tmp/val3"

#./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val3 --name=myNode3 --validator --port=30335 --ws-port=9946 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWSMNLnMk1CpwEn8N8UHe7dnHyYYYLUqiSioQv1f5xkeN5 --node-key=e8598b95d16d19309806f48443aa2f4fdd77a181a955a10196884f10a584c5a9


./target/debug/frontier-template-node --chain=./customSpecRaw.json --sync=Full --base-path=/tmp/val3 --name=myNode3 --validator --execution=Native  --port=30335 --rpc-port=9935 --ws-port=9946 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --rpc-methods=unsafe --node-key=$nodekey 
#--bootnodes=/ip4/68.79.37.147/tcp/30333/p2p/12D3KooWCzKMjUXE5svQCZxGQJtznwHyYYFdU3xnBNUbSmDBZa7z

#./target/debug/frontier-template-node --chain=./customSpecRaw.json --base-path=/tmp/val3 --name=myNode3 --validator --execution=Native  --port=30335 --rpc-port=9935 --ws-port=9946 --unsafe-ws-external --unsafe-rpc-external --rpc-cors=all --rpc-methods=unsafe --bootnodes=/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 --node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e
