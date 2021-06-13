use futures::stream::StreamExt;
use futures::SinkExt;

use serde::Serialize;
use serde_json;

use std::io::Read;
use std::thread;

use bytes::Bytes;
use crossbeam_channel::Sender;
use flate2::read::GzDecoder;
use tokio::runtime;
use websocket_lite::{Message, Opcode, Result};

pub enum StockData {}

#[derive(Debug)]
pub struct DataSource {
    source: String,
    ping_times: u64,
    is_sub: bool,
}

enum SubStatus {
    Disable(u64),
    Enable,
    InSub,
}

#[derive(Serialize)]
struct Pong<'a> {
    pong: &'a serde_json::Number,
}

#[derive(Serialize)]
struct Sub<'a> {
    sub: &'a str,
    id: &'a str,
}

type AsyncClient =
    websocket_lite::AsyncClient<Box<dyn websocket_lite::AsyncNetworkStream + Unpin + Send + Sync>>;

impl DataSource {
    pub fn new(source: String) -> DataSource {
        DataSource {
            source,
            ping_times: 0,
            is_sub: false,
        }
    }

    async fn process_ping(
        &mut self,
        json: &serde_json::Map<String, serde_json::Value>,
        ws_stream: &mut AsyncClient,
    ) {
        if let Some(num) = json.get("ping") {
            if let serde_json::Value::Number(num) = num {
                let pong = Pong { pong: num };
                let pong_rs = serde_json::to_string(&pong);
                let pong_rs = Message::text(pong_rs.unwrap());
                println!("{:?}", &pong_rs);
                if !self.enable_sub() {
                    self.ping_times += 1;
                }
                ws_stream.send(pong_rs).await;
            }
        }
    }

    async fn sub(&mut self, topic: &str, id: &str, ws_stream: &mut AsyncClient) {
        let sub_rs = serde_json::to_string(&Sub { sub: topic, id: id });
        let sub_rs = Message::text(sub_rs.unwrap());
        println!("{:?}", &sub_rs);
        ws_stream.send(sub_rs).await;
        self.is_sub = true;
    }

    #[inline]
    fn enable_sub(&self) -> bool {
        if self.ping_times >= 1 && self.is_sub == false {
            true
        } else {
            false
        }
    }

    pub fn run(self, sender: Sender<StockData>) {
        let mut inner = self;
        thread::spawn(move || {
            let rt = runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap();
            rt.block_on(async {
                let builder = websocket_lite::ClientBuilder::new(&inner.source).unwrap();
                let mut ws_stream = builder.async_connect().await.unwrap();
                loop {
                    let ws_msg: Option<Result<Message>> = ws_stream.next().await;
                    if ws_msg.is_none() {
                        return;
                    }
                    let ws_msg = ws_msg.unwrap();
                    if let Err(e) = ws_msg {
                        eprintln!("message recv error, detail:{}", e);
                        return;
                    }
                    let msg = ws_msg.unwrap().into_data();
                    let mut gz_data = GzDecoder::new(&msg[..]);
                    let mut data = String::new();
                    if let Err(e) = gz_data.read_to_string(&mut data) {
                        eprintln!("message gzip decode error, detail {}", e);
                        return;
                    }
                    let sval = serde_json::from_str::<serde_json::Value>(&data);
                    if let Err(e) = sval {
                        eprintln!("parse json error, detail {}", e);
                        return;
                    }
                    println!("{:?}", sval);
                    if let serde_json::Value::Object(json) = sval.unwrap() {
                        if json.get("ping").is_some() {
                            inner.process_ping(&json, &mut ws_stream).await;
                        }
                    }
                    if inner.enable_sub() {
                        inner
                            .sub("market.ethusdt.kline.1min", "id1", &mut ws_stream)
                            .await;
                    }
                }
            });
        });
    }
}
