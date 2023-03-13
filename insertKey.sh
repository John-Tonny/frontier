# !/bin/bash

suri=$1
mypath=$2
name=frontier-template-node
chain=$3
cmd=$4


if [[ "x${cmd}" == "x" ]]; then
  cmd=./target/debug/$name
fi

echo $suri
echo $mypath
echo $chain
echo $cmd

$cmd key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type aura

$cmd key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type babe

$cmd key insert --base-path $mypath --chain $chain --scheme Ed25519 --suri "$suri"  --key-type gran

$cmd key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type stak

$cmd key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type imon

$cmd key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type acco

$cmd key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type audi

$cmd key insert --base-path $mypath --chain $chain --scheme Sr25519 --suri "$suri"  --key-type mnod
