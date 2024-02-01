#![allow(dead_code, unused_imports)]//功能:没有使用的代码或模块不警告
#[macro_use]
extern crate chrono;
extern crate time;
extern crate wasm_bindgen_test;
extern crate anyhow;
extern crate num_cpus;
extern crate lazy_static;
extern crate clap;
// extern crate log;
// extern crate sled;
//extern crate convert_case;
mod common;
mod config;
mod outputcolor;
mod setting;
mod sled;
mod structure;
mod myyew;
mod my_thread;
mod my_gui;
mod redistest;
mod socketsub;

use anyhow::Result;
use anyhow::anyhow;
use axum::response::Html;
use tokio::sync::RwLock;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::iter::repeat;
use std::mem::size_of_val;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Write;

use crate::common::SummarizedTickData;
use crate::common::UidgenService;
use crate::config::Config;
// use crate::redistest::redisclustertest::RedisClusterClient;

use self::chrono::prelude::*;
use self::chrono::offset::Local;
use chrono::{DateTime, TimeZone, Utc};


use convert_case::{Case, Casing};
use dashmap::DashMap;

use time::strptime;
use ansi_term::Colour::Red;
// use time::*;

use std::time::Instant;
use std::time::*;
// use thread_id::{self};



use tokio::sync::broadcast;

use cached::{Cached, TimedSizedCache};
use setting::Settings;
use tokio::runtime;
// use myyew::Model;
// use loom::sync::atomic::AtomicUsize;
// use loom::sync::atomic::Ordering::{Acquire, Relaxed, Release};
// use loom::sync::Arc;
// use loom::thread;

use console::{style, Term};

use outputcolor::{console, print_da, write_chars, test as TEST};
use crate::outputcolor::do_stuff;
use structure::Structure;

use axum::{
    routing::get,
    Router,
};
use local_ip_address::local_ip;

// use gtk::prelude::*;
// use gtk::{glib, Application};
#[tokio::main]
async fn main() {
    if let Err(e) = socketsub::socket3::client().await {
        println!("{:?}", e);
    }
}