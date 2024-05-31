#!/bin/bash

echo "searching for pis..."

for i in $(seq 0 20)
do
  sleep 15
  COUNT=$(sudo nmap -sP 192.168.4.0/24 | grep Raspberry | wc -l)
  if [[ "$COUNT" == "2" ]]; then
    echo "all pis online"
    exit 0
  elif [[ "$COUNT" == "1" ]]; then
    echo "one pi online. searching..."
  else
    echo "searching..."
  fi
done