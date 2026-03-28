pub use waynest;
pub use waynest_client;
pub use waynest_protocols;

use futures_core::Stream;
use futures_sink::Sink;
use waynest::{Connection, Message, ProtocolError};

pub struct Client {}

impl Connection for Client {
    type Error = ProtocolError;

    fn fd(&mut self) -> Result<std::os::unix::prelude::OwnedFd, <Self as Connection>::Error> {
        todo!()
    }
}

impl Stream for Client {
    type Item = Result<Message, ProtocolError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}

impl Sink<Message> for Client {
    type Error = ProtocolError;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn start_send(self: std::pin::Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        todo!()
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }
}
