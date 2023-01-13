# !/bin/bash

suri=$1
mypath=$2
name=frontier-template-node
chain=customSpecRaw.json

echo $suri
echo $mypath

./target/debug/$name key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type aura

./target/debug/$name key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type babe

./target/debug/$name key insert --base-path $mypath --chain $chain --scheme Ed25519 --suri "$suri"  --key-type gran

./target/debug/$name key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type stak

./target/debug/$name key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type imon

./target/debug/$name key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type acco

./target/debug/$name key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type audi

./target/debug/$name key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type demo
