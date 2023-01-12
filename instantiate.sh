#!/bin/bash
junod tx wasm instantiate "$1" '{}' --node https://rpc.uni.junonetwork.io:443 --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 -b block --from mikereg -o json --label "CronCat-rules-alpha" --admin "$(junod keys show mikereg -a)" -y | jq | head -n 42
