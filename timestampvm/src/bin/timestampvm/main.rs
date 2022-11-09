pub mod genesis;
pub mod vm_id;

use std::io;

use timestampvm::vm;
use tokio::sync::broadcast::{self, Receiver, Sender};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (stop_ch_tx, stop_ch_rx): (Sender<()>, Receiver<()>) = broadcast::channel(1);

    let vm_server =
        avalanche_types::subnet::rpc::vm::server::Server::new(vm::Vm::new(), stop_ch_tx);

    avalanche_types::subnet::rpc::plugin::serve(vm_server, stop_ch_rx).await
}
