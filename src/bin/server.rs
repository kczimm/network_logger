use tonic::{transport::Server, Request, Response, Status};

use network_logger::proto::logger_server::{Logger, LoggerServer};
use network_logger::proto::{LevelReply, LevelRequest, LogReply, LogRequest};

#[derive(Debug, Default)]
pub struct MyLogger {}

#[tonic::async_trait]
impl Logger for MyLogger {
    async fn log(&self, request: Request<LogRequest>) -> Result<Response<LogReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = LogReply {
            message: "".to_owned(),
        };

        Ok(Response::new(reply))
    }

    async fn level(&self, _request: Request<LevelRequest>) -> Result<Response<LevelReply>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let logger = MyLogger::default();

    Server::builder()
        .add_service(LoggerServer::new(logger))
        .serve(addr)
        .await?;

    Ok(())
}
