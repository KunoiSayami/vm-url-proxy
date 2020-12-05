/*
 ** Copyright (C) 2020 KunoiSayami
 **
 ** This file is part of vm-url-proxy and is released under
 ** the AGPL v3 License: https://www.gnu.org/licenses/agpl-3.0.txt
 **
 ** This program is free software: you can redistribute it and/or modify
 ** it under the terms of the GNU Affero General Public License as published by
 ** the Free Software Foundation, either version 3 of the License, or
 ** any later version.
 **
 ** This program is distributed in the hope that it will be useful,
 ** but WITHOUT ANY WARRANTY; without even the implied warranty of
 ** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 ** GNU Affero General Public License for more details.
 **
 ** You should have received a copy of the GNU Affero General Public License
 ** along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use webbrowser;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let body= String::from_utf8_lossy(&buffer);


    let tmp = body.trim_matches(char::from(0)).split("\r\n\r\n");
    let website_address = tmp.last().unwrap();

    match webbrowser::open(website_address) {
        Ok(_) => println!("Opened {}", website_address),
        Err(e) => eprintln!("Got error {:?}", e)
    }

    let response = format!(
        "HTTP/1.1 204 No Content\r\nContent-Type: text/html"
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}