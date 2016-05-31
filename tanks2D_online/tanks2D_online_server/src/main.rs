
extern crate rustc_serialize;
extern crate bincode;

pub mod configuration;
mod net;

use configuration::{ServerConfig, PATH_TO_CONFIG_FILE};
use net::UdpSocketWrapper;

fn main() {
    
    let config = ServerConfig::from_file(PATH_TO_CONFIG_FILE).unwrap();
    println!("{:?}", config);
    let socket = UdpSocketWrapper::bind(config);
    
}
