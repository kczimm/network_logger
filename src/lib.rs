use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use once_cell::sync::OnceCell;

use proto::logger_client::LoggerClient;
use proto::LogRequest;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

pub mod proto {
    tonic::include_proto!("log");
}

static LOGGER: OnceCell<SimpleLogger> = OnceCell::new();

pub fn init() -> Result<(), SetLoggerError> {
    let (tx, mut rx) = unbounded_channel();
    let logger = SimpleLogger { tx };
    LOGGER.set(logger).expect("set logger only once");

    tokio::spawn(async move {
        let mut client = LoggerClient::connect("http://[::1]:50051").await.unwrap();

        while let Some(message) = rx.recv().await {
            eprintln!("{}", message);
            let request = tonic::Request::new(LogRequest { message });
            let _response = client.log(request).await.unwrap();
        }
    });

    log::set_logger(LOGGER.get().unwrap()).map(|()| log::set_max_level(LevelFilter::Info))
}

#[derive(Debug)]
struct SimpleLogger {
    tx: UnboundedSender<String>,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let s = format!("{} - {}", record.level(), record.args());

            self.tx.send(s).unwrap();
        }
    }

    fn flush(&self) {}
}
