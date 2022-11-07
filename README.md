
[<img alt="crates.io" src="https://img.shields.io/crates/v/timestampvm.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/timestampvm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-timestampvm-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/timestampvm)
![Github Actions](https://github.com/ava-labs/timestampvm-rs/actions/workflows/test-and-release.yml/badge.svg)

# timestampvm-rs

Timestamp VM in Rust

See [`tests/e2e`](tests/e2e/src/tests/mod.rs) for full end-to-end tests.

## Example

```bash
# to build the timestampvm plugin, run e2e tests, and keep the network running
# add NETWORK_RUNNER_SKIP_SHUTDOWN=1 to tests.e2e.sh to shut down network afterwards
./scripts/build.release.sh \
&& VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh
```

To test timestampvm API manually:

```bash
# "tGas3T58KzdjcJ2iKSyiYsWiqYctRXaPTqBCA11BqEkNg8kPc" is the Vm Id
# e.g., timestampvm vm-id timestampvm
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "ping",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/vm/tGas3T58KzdjcJ2iKSyiYsWiqYctRXaPTqBCA11BqEkNg8kPc/static

# {"jsonrpc":"2.0","result":{"success":true},"id":1}
```

```bash
# "22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "ping",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA/rpc

# {"jsonrpc":"2.0","result":{"success":true},"id":1}
```

```bash
# to get genesis block
# "22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "last_accepted",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA/rpc

# {"jsonrpc":"2.0","result":{"id":"ucteGvjauToAchS7VhLj61LRsgu6hpr1uqmafxAKM8XJambgN"},"id":1}

# "22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "get_block",
    "params" : [{"id":"ucteGvjauToAchS7VhLj61LRsgu6hpr1uqmafxAKM8XJambgN"}]
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA/rpc

# {"jsonrpc":"2.0","result":{"block":{"data":"0x39457835325173437661","height":0,"parent_id":"11111111111111111111111111111111LpoYY","timestamp":0}},"id":1}
```

```bash
# to propose data
echo 1 | base64 | tr -d \\n
# MQo=

curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "propose_block",
    "params" : [{"data":"MQo="}]
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA/rpc

# {"jsonrpc":"2.0","result":{"success":true},"id":1}
```

```bash
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "last_accepted",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA/rpc

# {"jsonrpc":"2.0","result":{"id":"ucteGvjauToAchS7VhLj61LRsgu6hpr1uqmafxAKM8XJambgN"},"id":1}

curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "get_block",
    "params" : [{"id":"ucteGvjauToAchS7VhLj61LRsgu6hpr1uqmafxAKM8XJambgN"}]
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/22VVUe644h17b68LtbyLd81S4ACXNhWjsrMeikV7DiHHdseCXA/rpc

# TODO
```
