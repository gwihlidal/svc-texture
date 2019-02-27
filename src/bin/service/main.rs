#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate env_logger;
extern crate futures;
extern crate futures_cpupool;
extern crate prost;
extern crate prost_types;
extern crate tokio;
extern crate tower_grpc;
extern crate tower_h2;
extern crate tower_util;
#[macro_use]
extern crate log;
extern crate h2;
extern crate svc_texture;
#[macro_use]
extern crate serde_derive;

use futures::sync::mpsc;
use futures::{future, Future, Sink, Stream};
use h2::server::Builder;
use sha2::{Digest, Sha256};
use std::collections::hash_map::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use svc_texture::error::{Error, ErrorKind, Result};
use svc_texture::identity::compute_data_identity;
use svc_texture::proto;
use svc_texture::utilities::{
    any_as_u8_slice, compute_file_identity, compute_identity, path_exists, read_file, TempDir,
    TempFile, BUILD_ID,
};
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_grpc::Error as GrpcError;
use tower_grpc::Request as GrpcRequest;
use tower_grpc::Response as GrpcResponse;
use tower_h2::Server;

mod process;
use crate::process::*;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

// TODO: Remove hardcoding
static OPTIMAL_WINDOWING: bool = false;

#[derive(Clone, Debug)]
struct ServiceBackend {
    context: Arc<ServiceContext>,
}

#[derive(Debug)]
struct ServiceContext {
    storage_path: PathBuf,
    transform_path: PathBuf,
    temp_path: PathBuf,
}

impl ServiceBackend {}

unsafe impl Send for ServiceBackend {}
unsafe impl Sync for ServiceBackend {}

impl proto::service::server::Texture for ServiceBackend {
    type QueryStream = Box<Stream<Item = proto::common::StorageState, Error = GrpcError> + Send>;
    type QueryFuture = future::FutureResult<GrpcResponse<Self::QueryStream>, GrpcError>;

    type UploadFuture = Box<
        future::Future<Item = GrpcResponse<proto::common::StorageIdentity>, Error = GrpcError>
            + Send,
    >;

    type DownloadStream =
        Box<Stream<Item = proto::common::StorageContent, Error = GrpcError> + Send>;
    type DownloadFuture = future::FutureResult<GrpcResponse<Self::DownloadStream>, GrpcError>;

    fn query(
        &mut self,
        request: GrpcRequest<tower_grpc::Streaming<proto::common::StorageIdentity>>,
    ) -> Self::QueryFuture {
        let context = self.context.clone();
        future::ok(GrpcResponse::new(Box::new(request.into_inner().map(
            move |identity| {
                let content_path = context.storage_path.join(&identity.sha256_base58);
                match std::fs::metadata(&content_path) {
                    Ok(ref meta_data) => {
                        proto::common::StorageState {
                            identity: Some(identity),
                            exists: true,
                            length: meta_data.len(),
                            meta_data: HashMap::new(), // TODO: Add support
                        }
                    }
                    Err(_) => proto::common::StorageState {
                        identity: Some(identity),
                        exists: false,
                        length: 0,
                        meta_data: HashMap::new(),
                    },
                }
            },
        ))))
    }

    fn upload(
        &mut self,
        request: tower_grpc::Request<tower_grpc::Streaming<proto::common::StorageContent>>,
    ) -> Self::UploadFuture {
        let context = self.context.clone();

        struct UploadContext {
            writer: Option<Cursor<Vec<u8>>>,
            content_encoding: String,
            content_type: String,
        }

        let upload_context = UploadContext {
            writer: None,
            content_encoding: String::new(),
            content_type: String::new(),
        };

        let response = request
            .into_inner()
            .map_err(|e| {
                println!("  !!! err={:?}", e);
                e
            })
            // Iterate over all request messages, building up the storage entry
            .fold(
                (upload_context, None),
                move |(mut upload_context, _last_request), request| {
                    // Check for first incoming rpc message for the stream
                    if upload_context.writer.is_none() {
                        upload_context.writer = Some(Cursor::new(Vec::with_capacity(
                            request.total_length as usize,
                        )));
                        assert!(upload_context.writer.is_some());
                        upload_context.content_encoding = request.encoding.clone();
                        upload_context.content_type = request.type_.clone();
                    }

                    if let Some(writer) = &mut upload_context.writer {
                        if let Err(err) = writer.write(&request.chunk_data) {
                            println!("Error occurred writing chunk bytes! {}", err);
                        }
                    }

                    Ok::<_, tower_grpc::Error>((upload_context, Some(request)))
                },
            )
            // Map the response to a gRPC response
            .map(move |(mut upload_context, _)| {
                if let Some(writer) = &mut upload_context.writer {
                    let bytes_received = writer.position();
                    let data_buffer: &Vec<u8> = writer.get_ref();
                    let data_identity = compute_data_identity(&data_buffer);

                    let content_path = context.storage_path.join(&data_identity.txt);
                    let content_file = File::create(&content_path).expect("Unable to create file");
                    let mut content_writer = BufWriter::new(content_file);
                    content_writer
                        .write_all(&data_buffer)
                        .expect("Unable to write data");
                    GrpcResponse::new(proto::common::StorageIdentity {
                        sha256_base58: data_identity.txt,
                    })
                } else {
                    GrpcResponse::new(proto::common::StorageIdentity::default())
                }
            });
        Box::new(response)
    }

    fn download(
        &mut self,
        request: GrpcRequest<proto::service::DownloadRequest>,
    ) -> Self::DownloadFuture {
        use smush::Encoding;
        use std::str::FromStr;

        let context = self.context.clone();

        let identity = match request.get_ref().identity {
            Some(ref identity) => identity.sha256_base58.to_owned(),
            None => unimplemented!(),
        };
        let encoding = match Encoding::from_str(&request.get_ref().encoding) {
            Ok(encoding) => encoding,
            Err(_err) => Encoding::Identity,
        };

        let content_path = context.storage_path.join(&identity);
        let content_data = read_file(&content_path).unwrap();

        let meta_chunk = proto::common::StorageContent {
            identity: Some(proto::common::StorageIdentity {
                sha256_base58: identity,
            }),
            encoding: "identity".to_string(),
            type_: "application/octet-stream".to_string(),
            total_length: content_data.len() as u64,
            ..Default::default()
        };

        let download_chunks: Vec<proto::common::StorageContent> = content_data
            .chunks(1024 * 1024)
            //.chunks(32 * 1024)
            .map(|chunk_data| {
                let mut request = meta_chunk.clone();
                request.chunk_data = chunk_data.to_vec();
                request
            })
            .collect();

        let download_chunk_stream = futures::stream::iter_ok(download_chunks);
        future::ok(GrpcResponse::new(Box::new(download_chunk_stream)))
    }
}

fn make_process_error(name: &str, message: &str) -> proto::service::ProcessOutput {
    proto::service::ProcessOutput {
        name: name.to_string(),
        output: String::new(),
        errors: message.to_string(),
        identity: None,
    }
}

fn run_service(backend: ServiceBackend, port: u16) {
    let addr = format!("0.0.0.0:{}", port).parse().unwrap();
    println!("Texture build service listening on: {}", addr);
    let bind = TcpListener::bind(&addr).expect("bind");

    let new_service = proto::service::server::TextureServer::new(backend);
    let mut h2_settings = h2::server::Builder::new();
    if OPTIMAL_WINDOWING {
        h2_settings.initial_window_size(65536 * 2048); // for an RPC
        h2_settings.initial_connection_window_size(65536 * 2048); // for a connection
    }
    let mut h2 = Server::new(new_service, h2_settings, DefaultExecutor::current());

    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = h2.serve(sock);
            tokio::spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    tokio::run(serve)
}

fn main() {
    use dotenv;
    dotenv::from_filename("service.env").expect("Failed to read .env file");
    env_logger::init();

    println!(
        "Initializing texture build service [version: {} - identity: {}]",
        VERSION.unwrap_or("unknown"),
        *BUILD_ID
    );

    let storage_path = match env::var("STORAGE_PATH") {
        Ok(path) => path.to_string(),
        Err(_) => "./.storage".to_string(),
    };

    let transform_path = match env::var("TRANSFORM_PATH") {
        Ok(path) => path.to_string(),
        Err(_) => "./.transform".to_string(),
    };

    let temp_path = match env::var("TEMP_PATH") {
        Ok(path) => path.to_string(),
        Err(_) => "./.temp".to_string(),
    };

    std::fs::create_dir_all(&storage_path).unwrap();
    std::fs::create_dir_all(&transform_path).unwrap();
    std::fs::create_dir_all(&temp_path).unwrap();

    // Create service context
    let context = Arc::new(ServiceContext {
        storage_path: Path::new(&storage_path)
            .to_path_buf()
            .canonicalize()
            .unwrap(),
        transform_path: Path::new(&transform_path)
            .to_path_buf()
            .canonicalize()
            .unwrap(),
        temp_path: Path::new(&temp_path).to_path_buf().canonicalize().unwrap(),
    });

    println!("STORAGE_PATH: {:?}", &context.storage_path);
    println!("TRANSFORM_PATH: {:?}", &context.transform_path);
    println!("TEMP_PATH: {:?}", &context.temp_path);

    // Create service backend
    let backend = ServiceBackend { context };

    // Launch the service!
    run_service(backend, 63998);
}
