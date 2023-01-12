#!/bin/bash

junod tx wasm execute juno1wqhttvnv8xcqs55wasu2s49dxmgyx5jpygp4n9p0tndc8kx7nenqsr0ev3 '{"toggle":{}}' -o json --from mikereg --gas-prices 0.025ujunox --gas auto --gas-adjustment 1.3 --broadcast-mode block -y | jq | head -n 10
