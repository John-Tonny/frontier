rm -rf /tmp/dave
./insertKey.sh "//Dave" "/tmp/dave"


RUST_LOG="info,runtime::ocw-unsigtx=debug" \
./target/debug/node-template \
--base-path /tmp/dave \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
--chain=local \
--dave \
--node-key 0000000000000000000000000000000000000000000000000000000000000003 \
--port 30335 \
--rpc-port 9935 \
--ws-port 9946 \
--no-telemetry \
--rpc-external \
--rpc-cors=all \
--rpc-methods=Unsafe \
--ws-external \
--execution=Native \
--enable-offchain-indexing=true
