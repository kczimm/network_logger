use log::{log, Level};
use network_logger;

#[tokio::main]
async fn main() {
    network_logger::init().unwrap();
    loop {
        log!(Level::Info, "new message");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
