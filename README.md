# timestampvm-rs

Timestamp VM in Rust

See [`tests/e2e`](tests/e2e) for full end-to-end tests.

## Example


```bash
./scripts/build.release.sh

NETWORK_RUNNER_SKIP_SHUTDOWN=1 \
VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh

VM_PLUGIN_PATH=$(pwd)/target/release/timestampvm \
./scripts/tests.e2e.sh
```
