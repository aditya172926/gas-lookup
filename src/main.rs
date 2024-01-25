mod lib;
use tokio::runtime::Builder;

pub fn main() {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            lib::gas_spent().await;
        })
}