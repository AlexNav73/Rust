
extern crate rustc_serialize;
extern crate bincode;

mod net;
mod data;

use net::configuration::{ServerConfig, PATH_TO_CONFIG_FILE};
use net::UdpSocketWrapper;

use data::SimpleTestObject;

fn main() {
    
    let config = ServerConfig::from_file(PATH_TO_CONFIG_FILE).unwrap();
    println!("{:?}", config);
    let socket = UdpSocketWrapper::bind(config);
    
}
