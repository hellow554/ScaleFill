use std::ops::Deref;

use futures_util::{SinkExt, StreamExt};
use num::Rational32;
use serde::{Deserialize, Serialize};
use tokio::{
    net::TcpStream,
    sync::{mpsc, watch},
};
use tokio_websockets::{Message, WebsocketStream};

use crate::scale::Unit;

use super::WsMessage;

#[derive(Serialize, Deserialize)]
enum Command {
    Start,
    Stop,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum Exchange<'a> {
    Measurement { value: Rational32, unit: Unit },
    Command { cmd: Command },
    Error { what: &'a str },
}

impl<'a> From<&'a WsMessage> for Exchange<'a> {
    fn from(value: &'a WsMessage) -> Self {
        match value {
            WsMessage::Measurement(m) => Self::Measurement {
                value: m.0,
                unit: m.1,
            },
            WsMessage::Error(e) => Self::Error { what: e.as_str() },
        }
    }
}

// fn x(msg: Message) {
//     msg.
// }

// fn x(msg: Message) -> Exchange<'static> {

//     // match msg {
//     //     WsMessage::Measurement(ms) => Exchange::Measurement {
//     //         value: ms.0,
//     //         unit: ms.1,
//     //     },
//     //     WsMessage::Error(_) => todo!(),
//     // }
// }

pub(super) async fn handle_client(
    mut stream: WebsocketStream<TcpStream>,
    sender: mpsc::Sender<WsMessage>,
    mut recv: watch::Receiver<WsMessage>,
) {
    println!("Client connected");
    let (mut w, mut r) = stream.split();
    'outer: loop {
        tokio::select! {
            Some(Ok(msg)) = r.next() => {
                if msg.is_close() {
                    break 'outer;
                }
                let Some(txt) = msg.as_text() else { continue; };
                dbg!(&msg);
                let Ok(msg) = serde_json::from_str::<Exchange>(txt) else { continue; };
            },

            // }
            Ok(()) = recv.changed() => {
                let txt = {
                    // we have to take care that recv does not live across the await berrier
                    let tmp = recv.borrow();
                    let msg: Exchange = tmp.deref().into();
                    Message::text(serde_json::to_string(&msg).unwrap())
                };
                if let Err(e) = w.send(txt).await {
                    // probably disconnected
                    dbg!(e);
                    break 'outer;
                }
            }
            else => {
                println!("Got to else clause");
                break 'outer;
            }
        }
    }
    println!("Client disconnected");
}
