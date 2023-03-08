# !/bin/bash

DEBUGINFO=/opt/debug.log
PIDFILE=/opt/alice.pid

pid=$(cat $PIDFILE)

used=$(free -m |grep "Mem" | awk '{print $ 3}')
used1=$(cat /proc/$pid/status |grep "VmRSS" | awk '{print $ 2}')

time=$(date)

echo "mem used:" $used  "MB --- " $used1 "KB on " $time
echo "mem used:" $used  "MB --- " $used1  "KB on " $time >> $DEBUGINFO 

#sed -i '20c #rm -rf ${BASEPATH}/chains' /usr/local/bin/start-frontier.sh

if (( $used >= 700 ))
then
	echo "restart frontier on " $time " from mem " $used "MB --- " $used1 "KB"
	echo "restatt frontier on " $time " from mem " $used "MB --- " $used1 "KB"  >> $DEBUGINFO
	sudo monit restart frontier
fi

if (( $used1 >= 700000 ))
then
	echo "restart frontier on " $time " from mem " $used "MB --- " $used1 "KB"
	echo "restatt frontier on " $time " from mem " $used "MB --- " $used1 "KB"  >> $DEBUGINFO
	sudo monit restart frontier
fi
