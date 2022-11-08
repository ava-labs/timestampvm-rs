
[<img alt="crates.io" src="https://img.shields.io/crates/v/timestampvm.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/timestampvm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-timestampvm-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/timestampvm)
![Github Actions](https://github.com/ava-labs/timestampvm-rs/actions/workflows/test-and-release.yml/badge.svg)

# timestampvm-rs

`timestampvm-rs` is a virtual machine that can build blocks from a user-provided arbitrary data. It provides a minimal implementation of an Avalanche custom virtual machine (VM) in Rust using the Avalanche [Rust SDK](https://github.com/ava-labs/avalanche-types-rs).

See [`tests/e2e`](tests/e2e/src/tests/mod.rs) for full end-to-end tests.

## Example

```bash
# to build the timestampvm plugin, run e2e tests, and keep the network running
# add NETWORK_RUNNER_SKIP_SHUTDOWN=1 to tests.e2e.sh to shut down network afterwards
./scripts/build.release.sh \
&& VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh

# or specify the custom avalanchego binary
./scripts/build.release.sh \
&& VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh ~/go/src/github.com/ava-labs/avalanchego/build/avalanchego
```

To test timestampvm API manually:

```bash
# "tGas3T58KzdjcJ2iKSyiYsWiqYctRXaPTqBCA11BqEkNg8kPc" is the Vm Id
# e.g., timestampvm vm-id timestampvm
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "timestampvm.ping",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/vm/tGas3T58KzdjcJ2iKSyiYsWiqYctRXaPTqBCA11BqEkNg8kPc/static

# {"jsonrpc":"2.0","result":{"success":true},"id":1}
```

```bash
# "2wb1UXxAstB8ywwv4rU2rFCjLgXnhT44hbLPbwpQoGvFb2wRR7" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "timestampvm.ping",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/2wb1UXxAstB8ywwv4rU2rFCjLgXnhT44hbLPbwpQoGvFb2wRR7/rpc

# {"jsonrpc":"2.0","result":{"success":true},"id":1}
```

```bash
# to get genesis block
# "2wb1UXxAstB8ywwv4rU2rFCjLgXnhT44hbLPbwpQoGvFb2wRR7" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "timestampvm.lastAccepted",
    "params" : []
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/2wb1UXxAstB8ywwv4rU2rFCjLgXnhT44hbLPbwpQoGvFb2wRR7/rpc

# {"jsonrpc":"2.0","result":{"id":"SDfFUzkdzWZbJ6YMysPPNEF5dWLp9q35mEMaLa8Ha2w9aMKoC"},"id":1}

# "2wb1UXxAstB8ywwv4rU2rFCjLgXnhT44hbLPbwpQoGvFb2wRR7" is the blockchain Id
curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "timestampvm.getBlock",
    "params" : [{"id":"SDfFUzkdzWZbJ6YMysPPNEF5dWLp9q35mEMaLa8Ha2w9aMKoC"}]
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/2wb1UXxAstB8ywwv4rU2rFCjLgXnhT44hbLPbwpQoGvFb2wRR7/rpc

# {"jsonrpc":"2.0","result":{"block":{"data":"0x32596655705939524358","height":0,"parent_id":"11111111111111111111111111111111LpoYY","timestamp":0}},"id":1}
```

```bash
# to propose data
echo 1 | base64 | tr -d \\n
# MQo=

curl -X POST --data '{
    "jsonrpc": "2.0",
    "id"     : 1,
    "method" : "timestampvm.proposeBlock",
    "params" : [{"data":"MQo="}]
}' -H 'content-type:application/json;' 127.0.0.1:9650/ext/bc/2wb1UXxAstB8ywwv4rU2rFCjLgXnhT44hbLPbwpQoGvFb2wRR7/rpc

# {"jsonrpc":"2.0","result":{"success":true},"id":1}
```

## Avalanche Custom Virtual Machine in Subnet

The project presents how to build a custom VM in Rust meeting the following requirements:

1. The custom VM plug-in must be compiled to a binary, so that `avalanchego` can launch it as a sub-process plug-in.
2. The VM binary path is named in VM ID, hash type with 32 bytes (256 bits) size.
3. The VM binary must implement snowman "block.ChainVM" interface, to be registered via "rpcchainvm.Serve".
4. The VM binary must implement VM-specific services, served via URL path of the blockchain ID.
5. Optionally, the binary implements VM-specific static handlers, served via URL path of the VM ID.
