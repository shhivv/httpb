mod tcp_interface;
mod udp_webserver;

use std::thread;

fn main() {
    let tcp_handle = thread::spawn(tcp_interface::server::listen);
    let udp_handle = thread::spawn(udp_webserver::server::listen);

    tcp_handle.join().unwrap();
    udp_handle.join().unwrap();
}
