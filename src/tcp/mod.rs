use crate::data;
use crate::queries::bk;
use crate::{config, timeprintln};
use std::{net, str::FromStr};

fn create_listener(tcp_listen_address: net::SocketAddr) -> net::TcpListener {
    let listener = net::TcpListener::bind(tcp_listen_address).unwrap();
    let local_addr = listener.local_addr().unwrap();
    timeprintln!("TCP server listening on {}...", local_addr);
    listener
}

/// Start Metrobaza server loop.
pub fn start_server(config: config::Config) {
    let mut index = data::Index::new("r9k", &config);
    index.add(0b1001);
    index.add(0b1110);
    let mut tree = bk::Tree::new();
    for element in index.get_data() {
        tree.add(element);
    }
    println!("{:?}", tree.search(&0b1111u128, 2));

    let tcp_listen_address = net::SocketAddr::new(
        net::IpAddr::from_str(&config.tcp_listen_host).unwrap(),
        config.tcp_listen_port,
    );
    let listener = create_listener(tcp_listen_address);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        timeprintln!("Connection established!");
    }
}
