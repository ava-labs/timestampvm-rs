
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

```bash
# to test timestampvm API manually
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "ping",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/vm/tGas3T58KzdjcJ2iKSyiYsWiqYctRXaPTqBCA11BqEkNg8kPc
```

