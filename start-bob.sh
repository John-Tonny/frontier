rm -rf /tmp/bob
./insertKey.sh "//Bob" "/tmp/bob"

#./target/debug/node-template \
./target/debug/frontier-template-node \
--base-path /tmp/bob \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--chain=local \
--bob \
--node-key 0000000000000000000000000000000000000000000000000000000000000002 \
--port 30334 \
--rpc-port 9934 \
--ws-port 9945 \
--no-telemetry \
--rpc-external \
--rpc-cors=all \
--rpc-methods=Unsafe \
--ws-external \
--execution=Native \
--enable-offchain-indexing=true
