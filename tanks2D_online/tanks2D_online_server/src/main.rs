
extern crate rustc_serialize;
extern crate bincode;

pub mod configuration;
pub mod data;
pub mod net;
mod server;

use configuration::*;
use net::UdpSocketWrapper;
use data::{SimpleTestObject, SimpleTestObjectHandler};
use server::Server;

pub const PATH_TO_CONFIG_FILE: &'static str = "server_config.json";

fn main() {
    
    let builder = ServerConfBuilder::load(PATH_TO_CONFIG_FILE).unwrap();
    let conf = builder.create();
    println!("{:?}", conf);
    
    let mut server = Server::new(conf).unwrap();
    server.reg_handler::<SimpleTestObject, SimpleTestObjectHandler>(SimpleTestObjectHandler);
    
}
