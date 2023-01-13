# !/bin/bash

#./target/debug/frontier-template-node  build-spec  --chain local > customSpec.json
./target/debug/frontier-template-node  build-spec --disable-default-bootnode --chain local > customSpec.json
