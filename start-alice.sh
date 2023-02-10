rm -rf /tmp/alice
./insertKey.sh "//Alice" "/tmp/alice"

./target/debug/frontier-template-node \
--base-path /tmp/alice \
--chain=local \
--alice \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--no-telemetry \
--rpc-external \
--rpc-cors=all \
--rpc-methods=Unsafe \
--ws-external \
--execution=Native \
--enable-offchain-indexing=true \
--pruning=archive
