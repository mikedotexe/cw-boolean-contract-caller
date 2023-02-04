#!/bin/bash

starsd tx wasm execute stars13nl2lqe3gesdf7ld0z0fa9sadt5mgy98tq7xre53pnw5eyhs2pmsze3ffv '{"make_croncat_task":{"croncat_factory_address":"stars1frqpxtgdhq7psq9u97v32kc69zeq0etal8vmnmp0st68ye5928jq8437kv","boolean_address":"stars10skdjcaeqpn84fxssasd352nkwkgxd88mrxz9cvkdvj0tq62s0wq2re5nq"}}' --amount 1000000ustars --gas-prices 0.025ustars --gas-adjustment 1.7 --gas auto -b block --from mikereg -o json -y | jq | head -n 42
