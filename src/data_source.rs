use futures::stream::StreamExt;
use futures::SinkExt;

use serde::{Serialize, Deserialize};
use serde_json;

use std::io::Read;
use std::collections::HashMap;
use std::{thread, fmt};
use std::cell::RefCell;

use bytes::Bytes;
use crossbeam_channel::Sender;
use flate2::read::GzDecoder;
use tokio::runtime;
use websocket_lite::{Message, Opcode, Result};

pub enum StockData {}

#[derive(Debug)]
struct StockChannel {
    id: String,
    ch: String,
    status: String,
}


pub struct DataSource {
    source: String,
    pong_time: u64,
    id_seq: u64,
}

#[derive(Deserialize, Debug)]
struct Subbed {
    id: String,
    status: String,
    subbed: String,
    ts: u64,
}

struct Context {
    ch: HashMap<String, StockChannel>
}

impl Context {
    fn is_pub(&self, ch: &str) -> bool {
        self.ch.contains_key(ch)
    }
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
            pong_time: 0,
            id_seq: 1,
        }
    }


    async fn process_ping(
        &mut self,
        json: &serde_json::Map<String, serde_json::Value>,
        ws_stream: &mut AsyncClient,
    ) {
        if let Some(num) = json.get("ping") {
            if let serde_json::Value::Number(ref num) = num {
                let pong = Pong { pong: num };
                let pong_rs = serde_json::to_string(&pong);
                let pong_rs = Message::text(pong_rs.unwrap());
                ws_stream.send(pong_rs).await;
                self.pong_time = num.as_u64().unwrap();
            }
        }
    }

    async fn process_sub(
        &mut self,
        ctx: &mut Context,
        json: serde_json::Value,
    ) {
        let subbed: Subbed = serde_json::from_value(json).unwrap();
        ctx.ch.get_mut(&(subbed.subbed)).map(|val| {
            val.status = subbed.status
        });
    }

    async fn sub(&mut self, topic: &str, id: &str, ws_stream: &mut AsyncClient) {
        let sub_rs = serde_json::to_string(&Sub { sub: topic, id: id });
        let sub_rs = Message::text(sub_rs.unwrap());
        ws_stream.send(sub_rs).await.unwrap();
    }
    

    #[inline]
    fn is_pong(&self) -> bool {
        if self.pong_time > 0 {
            true
        } else {
            false
        }
    }

    fn next_id_seq(&mut self) -> u64 {
        self.id_seq += 1;
        self.id_seq
    }

    fn register_ch<'a>(&mut self, ctx: &'a mut Context, ch: &str) ->  &'a mut StockChannel {
        let id: String = format!("id{}", self.next_id_seq()).into();
        ctx.ch.entry(ch.into())
            .or_insert(StockChannel{id: id, ch: ch.into(), status: String::from("uninited")})
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
                let ctx = RefCell::new(Context{ch: HashMap::new()});
                loop {
                    let will_pub = "market.ethusdt.kline.1min";
                    let mut ctx_ref = ctx.borrow_mut();
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
                    let sval = match serde_json::from_str::<serde_json::Value>(&data) {
                        Err(e) => {
                            eprintln!("parse json error, detail {}", e);
                            return;
                        },
                        Ok(v) => v,
                    };
                    println!("{:?}", sval);
                    if let serde_json::Value::Object(ref json) = sval {
                        if json.get("ping").is_some() {
                            inner.process_ping(json, &mut ws_stream).await;
                        }
                        if json.get("subbed").is_some() {
                            inner.process_sub(&mut ctx_ref, sval).await;
                        }
                    }
                    if inner.is_pong() && !&ctx_ref.is_pub(will_pub) {
                        let rs = inner.register_ch(&mut ctx_ref, will_pub);
                        inner
                            .sub(&rs.ch, &rs.id, &mut ws_stream)
                            .await;
                    }
                    
                    
                }
            });
        });
    }
}
