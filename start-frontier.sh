# !/bin/bash
TAGS=$1
BASEPATH=/opt/.frontier-data
CHAIN=${BASEPATH}/testnet.json
#MNEMONICS="chapter also vapor circle work hold fiction discover reveal paddle chaos unable"
MNEMONICS="board famous portion crack idea glove page diary wool thunder dragon push"

PIDFILE=/opt/frontier.pid
LOGFILE=/opt/frontier.log

CMD=./target/debug/frontier-template-node

SYNCMODE=Full

run_program ()
{

  NODEKEY=$($CMD key inspect "${MNEMONICS}" | grep "Public key (hex)"| awk '{print $4;}')

  echo $NODEKEY
  
  if [[ "x${TAGS}" == "xreset" ]]; then
    echo "reset frontier"
    rm -rf ${BASEPATH}/chains
  fi

  ./insertKey.sh "$MNEMONICS" "$BASEPATH" "$CHAIN"

  nohup $CMD --chain=$CHAIN --base-path=$BASEPATH  --execution=Native --node-key=$NODEKEY --port=30333 --ws-port=9944 --rpc-port=9933 --rpc-cors=all --unsafe-ws-external --unsafe-rpc-external --sync=$SYNCMODE >$LOGFILE 2>&1 &

  #nohup $CMD --chain=$CHAIN --base-path=$BASEPATH --execution=Native --validator --node-key=$NODEKEY --port=30333 --ws-port=9944 --rpc-port=9933 --rpc-cors=all --unsafe-ws-external --unsafe-rpc-external --rpc-methods=unsafe --sync=Full >$LOGFILE 2>&1 &

  PID=$!
  if [ $? -eq 0 ]
  then
    echo "Successfully started frontier. PID=$PID. Logs are at $LOGFILE"
    echo $PID > $PIDFILE
  else
    echo "Could not start frontier - check logs at $LOGFILE"
  fi
}

/usr/local/bin/stop-frontier.sh
run_program

