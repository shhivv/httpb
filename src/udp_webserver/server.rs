use std::net::UdpSocket;

pub fn listen() {
    let socket = UdpSocket::bind("127.0.0.1:40010").unwrap();

    loop {
        let mut buf = vec![0;65535];

        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        let _buf = &mut buf[..amt];


        let contents = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>Hello!</title>
        </head>
        <body>
            <h1>Hello!</h1>
            <p>Hi from HTTPB</p>
        </body>
        </html>
        "#;

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        socket.send_to(response.as_bytes(), src).unwrap();
    }
}
