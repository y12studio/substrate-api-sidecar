use std::sync::Arc;
use actix_web::{web, App, HttpServer, Responder, error::BlockingError};
use substrate_subxt::{
    ClientBuilder,
    system::System,
};
use url::Url;
use sp_rpc::number::NumberOrHex;
use sp_core::H256;
use sp_storage::StorageKey;
use parking_lot::Mutex;
use serde::Serialize;

mod runtime;

use runtime::NodeRuntime;

type Client = substrate_subxt::Client::<NodeRuntime>;
type Block = sp_runtime::generic::Block<<NodeRuntime as System>::Header, <NodeRuntime as System>::Extrinsic>;
type SignedBlock = sp_runtime::generic::SignedBlock<Block>;

struct Server {
    /// subxt client
    client: Client,
    /// tokio runtime,
    tokio_rt: Mutex<tokio::runtime::Runtime>,
}


// timestamp

// {
//   "block": {
//     "header": {
//       "parentHash": "0x9972fe60815b23941772752051984763ce3d14a60cb9a6e82cdcc096c0b114c1",
//       "number": "0x953e1",
//       "stateRoot": "0x5f4ce36d4c9ba064908b9a36a4da03601f4e5bbe1fe62bde96b75ab79b9e7bbe",
//       "extrinsicsRoot": "0x18abfb73c7bd4b0149e67301b7bdbe2ce2fbd53237608ad5ae611a97c1426f48",
//       "digest": {
//         "logs": [
//           "0x06424142453402650000002868af0f00000000",
//           "0x05424142450101c45345701ebd275b169da0f2cc0c03d937fe4ca94ac1b0f16354ef8a8d96bb5d6f9a20a10b94d21f234ba4baddfdfe387d92aef7be31100a5da127318924d78a"
//         ]
//       }
//     },
//     "extrinsics": [
//       "0x280402000b8029199f6f01",
//       "0x1c0409007e4f2500",
//       "0x1004140000"
//     ]
//   },
//   "justification": null
// }

#[derive(Serialize)]
struct BlockSanitized {
    number: u32,
    block_hash: H256,
    parent_hash: H256,
    extrinsics: Vec<sp_runtime::OpaqueExtrinsic>,
    // TODO: timestamp!
}

impl BlockSanitized {
    pub fn new(block_hash: H256, signed: SignedBlock) -> Self {
        // let () = signed.block.deconstruct();
        // println!("{:?}", signed);

        let number = signed.block.header.number;
        let parent_hash = signed.block.header.parent_hash;
        let extrinsics = signed.block.extrinsics;

        BlockSanitized {
            block_hash,
            number,
            parent_hash,
            extrinsics,
        }
    }
}

impl Server {
    fn new() -> Self {
        let url: Url = "ws://127.0.0.1:9944".parse().expect("Fixed url, must parse; qed");
        let mut rt = tokio::runtime::Runtime::new().expect("Must be able to bootstrap Tokio Runtime");

        let client = loop {
            let client = ClientBuilder::<NodeRuntime>::new()
                .set_url(url.clone())
                .build();

            match rt.block_on(client) {
                Ok(client) => break client,
                Err(error) => println!("Failed to connect: {:?}", error),
            }
        };

        // println!("{:#?}", client.metadata());

        Server {
            client,
            tokio_rt: Mutex::new(rt),
        }
    }

    async fn hash(self: Arc<Self>, block: u32) -> Option<H256> {
        // use futures_util::compat::Future01CompatExt;

        let future = self.client.block_hash(Some(NumberOrHex::Number(block)));

        // future.compat().await.ok()?
        self.async_fut01(future).await.ok()?
    }

    async fn timestamp(self: Arc<Self>) -> Option<Vec<u8>> {
        let future = self.client.fetch(StorageKey("Timestamp".as_bytes().into()), None);

        self.async_fut01(future).await.ok()?
    }

    async fn finalized_head(self: Arc<Self>) -> Option<H256> {
        let future = self.client.finalized_head();

        self.async_fut01(future).await.ok()
    }

    async fn block(self: Arc<Self>, hash: Option<H256>) -> Option<SignedBlock> {
        let future = self.client.block(hash);

        self.async_fut01(future).await.ok()?
    }

    async fn async_fut01<F>(self: Arc<Self>, fut: F) -> Result<F::Item, BlockingError<F::Error>>
    where
        F: futures::Future + Send + 'static,
        F::Item: Send + 'static,
        F::Error: Send + std::fmt::Debug + 'static,
    {
        web::block(move || {
            self.tokio_rt.lock().block_on(fut)
        }).await
    }
}

async fn block(server: web::Data<Server>, number: web::Path<u32>) -> impl Responder {
    let server = server.into_inner();

    let hash = server.clone().hash(*number).await;
    let block = server.block(hash).await;

    let hash = match hash {
        Some(hash) => hash,
        None => return r#"{"error":"Block not found"}"#.to_owned(),
    };

    match block.and_then(|block| serde_json::to_string_pretty(&BlockSanitized::new(hash, block)).ok()) {
        Some(json) => json,
        None => "Failed to fetch block".to_string(),
    }
}

async fn latest_block(server: web::Data<Server>) -> impl Responder {
    let server = server.into_inner();

    let hash = server.clone().finalized_head().await;
    let block = server.block(hash).await;

    let hash = match hash {
        Some(hash) => hash,
        None => return r#"{"error":"Block not found"}"#.to_owned(),
    };

    match block.and_then(|block| serde_json::to_string_pretty(&BlockSanitized::new(hash, block)).ok()) {
        Some(json) => json,
        None => "Failed to fetch block".to_string(),
    }
}

async fn timestamp(server: web::Data<Server>) -> impl Responder {
    let ts = server.into_inner().timestamp().await;

    format!("{:?}", ts)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = web::Data::new(Server::new());

    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .service(web::resource("/block").to(latest_block))
            .service(web::resource("/block/").to(latest_block))
            .service(web::resource("/timestamp/").to(timestamp))
            .service(web::resource("/block/{number}").to(block))
            .service(web::resource("/block/{number}/").to(block))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}