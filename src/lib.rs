#![allow(unused_imports)]

extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate base58;
extern crate filebuffer;
extern crate futures;
extern crate futures_cpupool;
extern crate glob;
extern crate prost;
extern crate sha2;
#[macro_use]
extern crate prost_derive;
extern crate bincode;
extern crate brotli;
extern crate dotenv;
extern crate failure;
extern crate flate2;
extern crate lzma_rs;
extern crate prost_types;
extern crate tower_grpc;
extern crate uuid;
#[macro_use]
extern crate cfg_if;
extern crate chashmap;
extern crate scoped_threadpool;
#[macro_use]
extern crate serde_derive;
extern crate byteorder;
extern crate ptree;
extern crate serde;
extern crate twox_hash;
#[macro_use]
extern crate log;
extern crate fern;
//#[macro_use]
extern crate flatbuffers;
extern crate intel_tex;
extern crate structopt;

pub use self::gen::*;

// The generated code requires two tiers of outer modules so that references between
// modules resolve properly.
pub mod gen {
    pub mod proto {
        pub mod common {
            include!(concat!(env!("OUT_DIR"), "/common.rs"));
        }
        pub mod service {
            include!(concat!(env!("OUT_DIR"), "/service.rs"));
        }
    }
}

pub mod client;
pub mod compile;
pub mod encoding;
pub mod error;
pub mod identity;
pub mod utilities;

pub use crate::error::{pretty_error, Error, ErrorKind, Result};
