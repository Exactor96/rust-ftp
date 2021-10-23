extern crate clap;

use std::env;
use clap::{App, Arg};
use unftp_sbe_fs::ServerExt;


#[tokio::main]
async fn main() {
    let matches = App::new("Rust FTP Server")
                          .version("0.1.0")
                          .arg(Arg::with_name("port")
                               .short("p")
                               .long("port")
                               .value_name("server_port")
                               .help("Sets a server port")
                               .takes_value(true)
                               .default_value("2121"))
                           .arg(Arg::with_name("write")
                                .short("w")
                                .long("write")
                                .help("Add write permission"))
                          .get_matches();

    let writable = matches.is_present("write");
    let port = String::from(matches.value_of("port").unwrap_or("2121"));
    let server_string = format!("{}:{}", "0.0.0.0", port);
    let dir = env::current_dir().unwrap();

    let server = libunftp::Server::with_fs(dir)
        .greeting("Welcome to Rust FTP server")
        .passive_ports(50000..65535);
    server.listen(server_string).await;
    
}