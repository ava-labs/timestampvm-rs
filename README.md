
[<img alt="crates.io" src="https://img.shields.io/crates/v/timestampvm.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/timestampvm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-timestampvm-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/timestampvm)
![Github Actions](https://github.com/ava-labs/timestampvm-rs/actions/workflows/test-and-release.yml/badge.svg)

# timestampvm-rs

Timestamp VM in Rust

See [`tests/e2e`](tests/e2e) for full end-to-end tests.

## Example

```bash
# to build the timestampvm plugin
./scripts/build.release.sh

# to run e2e tests
VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh
```

```bash
# to build the timestampvm plugin
./scripts/build.release.sh

# to keep the network running after tests
NETWORK_RUNNER_SKIP_SHUTDOWN=1 \
VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
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
# "PkBR34m8NkDgkLnRD2Ke5bMaGPm1rNPN78YDnmdsxZhTts1pi" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "ping",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/PkBR34m8NkDgkLnRD2Ke5bMaGPm1rNPN78YDnmdsxZhTts1pi/rpc

# {"jsonrpc":"2.0","result":{"success":true},"id":1}
```

```bash
echo hello | base64
# aGVsbG8K

# "PkBR34m8NkDgkLnRD2Ke5bMaGPm1rNPN78YDnmdsxZhTts1pi" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "propose_block",
    "params" : [{"data":"aGVsbG8K"}]
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/PkBR34m8NkDgkLnRD2Ke5bMaGPm1rNPN78YDnmdsxZhTts1pi/rpc

# TODO
```

```bash
# "PkBR34m8NkDgkLnRD2Ke5bMaGPm1rNPN78YDnmdsxZhTts1pi" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "last_accepted",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/PkBR34m8NkDgkLnRD2Ke5bMaGPm1rNPN78YDnmdsxZhTts1pi/rpc

# {"jsonrpc":"2.0","result":{"id":"g25v3qDyAaHfR7kBev8tLUHouSgN5BJuZjy1BYS1oiHd2vres"},"id":1}
```

```bash
# "PkBR34m8NkDgkLnRD2Ke5bMaGPm1rNPN78YDnmdsxZhTts1pi" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "get_block",
    "params" : [{"id":"g25v3qDyAaHfR7kBev8tLUHouSgN5BJuZjy1BYS1oiHd2vres"}]
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/PkBR34m8NkDgkLnRD2Ke5bMaGPm1rNPN78YDnmdsxZhTts1pi/rpc

# TODO
```
