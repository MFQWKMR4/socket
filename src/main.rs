use std::env;
#[macro_use]
extern crate log;

mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;
mod util;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        util::exit("Specify [tcp|udp] [server|client] [addr:port].")
    }
    let protocol: &str = &args[1];
    let role: &str = &args[2];
    let address = &args[3];

    match protocol {
        "tcp" => match role {
            "server" => tcp_server::serve(address).unwrap_or_else(util::err),
            "client" => tcp_client::connect(address).unwrap_or_else(util::err),
            _ => util::exit("missing role.")
        }
        "udp" => match role {
            "server" => udp_server::serve(address).unwrap_or_else(util::err),
            "client" => udp_client::communicate(address).unwrap_or_else(util::err),
            _ => util::exit("missing role.")
        }
        _ => util::exit("missing protocol.")
    }
}