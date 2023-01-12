#!/bin/bash

junod q wasm contract-state smart juno1wqhttvnv8xcqs55wasu2s49dxmgyx5jpygp4n9p0tndc8kx7nenqsr0ev3 '{"get_value":{}}' -o json | jq | head -n 19
