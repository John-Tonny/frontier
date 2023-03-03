#!/bin/bash

PIDFILE=/opt/frontier.pid

stop_program ()
{
  pidfile=$1

  if [ -f $pidfile ]; then
    echo "Stopping Process - $pidfile. PID=$(cat $pidfile)"
    kill -9 $(cat $pidfile)
    rm -f $pidfile

    pkill -9 "frontier-template-node"
  else
    echo "Stopping Process - $pidfile."
  fi

}

stop_program $PIDFILE

