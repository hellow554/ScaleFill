mod client;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Stream;
use num::Rational32;
use tokio::{
    net::TcpListener,
    sync::{mpsc, watch},
};
use tokio_websockets::ServerBuilder;

use crate::scale::Measurement;

use self::client::handle_client;

use super::Result;

pub enum WsMessage {
    Measurement(Measurement),
    Error(String),
}

pub struct Ws {
    recv: mpsc::Receiver<WsMessage>,
    publ: watch::Sender<WsMessage>,
}

impl Ws {
    pub async fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel(5);
        let (publ, sub) = watch::channel(WsMessage::Measurement(Measurement(
            Rational32::default(),
            crate::scale::Unit::Kilogramm,
        )));
        let listener = TcpListener::bind("127.0.0.1:12321").await?;

        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let Ok(stream) = ServerBuilder::new().accept(stream).await else {
                    continue;
                };
                tokio::spawn(handle_client(stream, tx.clone(), sub.clone()));
            }
        });

        Ok(Self { recv: rx, publ })
    }

    pub fn publish_message(&self, msg: WsMessage) {
        drop(self.publ.send(msg));
    }
}

impl Stream for Ws {
    type Item = WsMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.recv).poll_recv(cx)
    }
}
