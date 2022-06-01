use std::{
    io::{Read, Write},
    net::{TcpListener, UdpSocket},
};

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:40012").unwrap();

    let udp = UdpSocket::bind("127.0.0.1:40011").unwrap();

    for mut stream in listener.incoming().flatten() {
        
        let mut resp_buf = vec![0; 65507];
        let mut req_buf = [0; 1024];

        stream.read(&mut req_buf).unwrap();

        udp.send_to(&req_buf, "127.0.0.1:40010").unwrap();

        let (amt, _src) = udp.recv_from(&mut resp_buf).unwrap();
        let buf = &mut resp_buf[..amt];

        stream.write(buf).unwrap();
        stream.flush().unwrap();
    }
}
