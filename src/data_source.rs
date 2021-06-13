use futures::stream::StreamExt;
use futures::SinkExt;

use serde::Serialize;
use serde_json;

use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

use tokio::runtime;
use websocket_lite::{Message, Opcode, Result};
use bytes::Bytes;
use flate2::read::GzDecoder;
use crossbeam_channel::Sender;


struct DataSourceInner {
    source: String,
    ping_times: u64,
    is_sub: bool,
}

pub enum StockData {

}

pub struct DataSource {
    inner: Arc<Mutex<DataSourceInner>>,
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

impl DataSourceInner {
    pub fn new(source: String) -> DataSourceInner {
        DataSourceInner {
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
}

impl DataSource {
    fn clone_soruce(&self) -> String {
        let guard = (*self.inner).lock().unwrap();
        (*guard).source.clone()
    }

    pub fn new(source: String) -> DataSource {
        DataSource {
            inner: Arc::new(Mutex::new(DataSourceInner::new(source))),
        }
    }
    pub fn run(&mut self, sender: Sender<StockData>) {
        let mut inner = self.inner.clone();
        let source = self.clone_soruce();
        thread::spawn(move || {
            let rt = runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap();
            rt.block_on(async {
                let builder = websocket_lite::ClientBuilder::new(&source).unwrap();
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
                    let mut inner_guard = (*inner).lock().unwrap();
                    if let serde_json::Value::Object(json) = sval.unwrap() {
                        if json.get("ping").is_some() {
                            (*inner_guard).process_ping(&json, &mut ws_stream).await;
                        }
                    }
                    if (*inner_guard).enable_sub() {
                        (*inner_guard)
                            .sub("market.ethusdt.kline.1min", "id1", &mut ws_stream)
                            .await;
                    }
                }
            });
        });
    }
}