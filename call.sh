#!/bin/bash

junod tx wasm execute juno1782gkddttuvay4rppcw67f2g4mfluvxdjs0pjysuqvzjq4gqx5hs9qrpa4 '{"make_croncat_task":{"croncat_manager_address":"juno174ncqgapq7fudqj64ut4ue47gevlqp93guecjelnkquruvnpjdgsuk046w","boolean_address":"juno1wqhttvnv8xcqs55wasu2s49dxmgyx5jpygp4n9p0tndc8kx7nenqsr0ev3"}}' --amount 1000000ujunox --from mikereg --node https://juno-testnet-rpc.polkachu.com:443 --chain-id uni-5 --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 -b block -y -o json | jq | head -n 42
