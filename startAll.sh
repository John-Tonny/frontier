# !/bin/bash

TAGS=$1

BASEPATH=/opt/.frontier-data
CHAIN=${BASEPATH}/testnet.json

CMD=./target/release/frontier-template-node

SYNCMODE=Full
VALIDATOR="--validator"
EXECUTION="--execution=Native"
PRUNING="--pruning=archive"
#OFFCHAINWORKER="--offchain-worker=never"
RPCMETHODS="--rpc-methods=unsafe"
WSEXTERNAL="--unsafe-ws-external"
RPCEXTERNAL="--unsafe-rpc-external"
RPCCORS="--rpc-cors=all"
DBCACHE="--db-cache=10"
INPEERS="--in-peers=5000"

run_program ()
{

  MNEMONICS=$1
  MYPATH=$BASEPATH/$2

  PIDFILE=$3
  LOGFILE=$4

  PORT=$5
  WSPORT=$6
  RPCPORT=$7

  NODEKEY=$8
  if [ "x${NODEKEY}" == "x" ]; then
    NODEKEY=$($CMD key inspect "${MNEMONICS}" | grep "Public key (hex)"| awk '{print $4;}')
  fi
  echo "nodekey:" $NODEKEY
  
  if [[ "x${TAGS}" == "xreset" ]]; then
    echo "reset frontier"
    rm -rf ${MYPATH}/chains
  fi

  ./insertKey.sh "$MNEMONICS" "$MYPATH" "$CHAIN" "$CMD"

  nohup $CMD --chain=$CHAIN --base-path=$MYPATH $OFFCHAINWORKER $PRUNING $EXECUTION $VALIDATOR $DBCACHE $INPEERS --node-key=$NODEKEY --port=$PORT --ws-port=$WSPORT --rpc-port=$RPCPORT $RPCCORS $WSEXTERNAL $RPCEXTERNAL $RPCMETHODS --sync=$SYNCMODE >$LOGFILE 2>&1 &

  PID=$!
  if [ $? -eq 0 ]
  then
    echo "Successfully started frontier. PID=$PID. Logs are at $LOGFILE"
    echo $PID > $PIDFILE
  else
    echo "Could not start frontier - check logs at $LOGFILE"
  fi
  echo ""
}

./stopAll.sh

#SEED="gather sunny debris place have suit diesel carpet example tomorrow seat famous"
#run_program "$SEED" "alice" "/opt/alice.pid" "/opt/alice.log" 30333 9944 9933

alice="c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a"
bob="6ce3be907dbcabf20a9a5a60a712b4256a54196000a8ed4050d352bc113f8c58"

run_program "//Alice" "alice" "/opt/alice.pid" "/opt/alice.log" 30333 9944 9933 $alice
run_program "//Bob" "bob" "/opt/bob.pid" "/opt/bob.log" 30334 9945 9934 $bob
