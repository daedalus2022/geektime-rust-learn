use async_prost::AsyncProstStream;
use dashmap::DashMap;
use futures::prelude::*;
use futures::stream::StreamExt;
use kv::command_request::RequestData;
use kv::KvError;
use kv::{CommandRequest, CommandResponse, Hset, Kvpair, Value};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // // 初始化日志
    tracing_subscriber::fmt::init();

    let addr = format!("{}:{}", "127.0.0.1", 9527);
    let linstener = TcpListener::bind(addr).await?;

    // 使用 DashMap 创建放在内存中的 kv store
    let table: Arc<DashMap<String, Value>> = Arc::new(DashMap::new());

    loop {
        debug!("得到一个客户端请求");
        let (stream, addr) = linstener.accept().await?;
        info!("Client {:?} connected", addr);

        // 复制 db， 让他在 tokio 任务中可以使用
        let db = table.clone();

        // 创建一个tokio 任务处理这个客户端
        tokio::spawn(async move {
            //使用 AsyncProstStream 来处理 TCP Frame
            //Frame: 两字节frame长度, 后面是protobuf 二进制
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();

            while let Some(Ok(msg)) = stream.next().await {
                info!("Got a new command: {:?}", msg);

                let resp = match msg.request_data {
                    Some(RequestData::Hset(cmd)) => hset(cmd, &db),
                    _ => unimplemented!(),
                };

                info!("Got response: {:?}", resp);

                // 把 CommandResponse 发送给客户端
                stream.send(resp).await.unwrap();
            }
        });
    }
}

fn hset(cmd: Hset, db: &DashMap<String, Value>) -> CommandResponse {
    match cmd.pair {
        Some(Kvpair {
            key,
            value: Some(v),
        }) => {
            let old: Value = db.insert(key, v).unwrap_or_default();
            old.into()
        }
        v => KvError::InvalidCommand(format!("hset: {:?}", v)).into(),
    }
}
