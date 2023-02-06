#!/bin/bash



for ((i=1;i<=10000;i++)); 
do
  curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "eth_getTransactionByHash", "params": ["0x4130634cefe1b39a6fe9c8000312c1b48ab07695e6de5cfbce1c53a933a36cbe"] }' http://68.79.37.147:9933/ >> txall.log
  echo "  $i   \r\n" >> txall.log
  sleep 6
done
