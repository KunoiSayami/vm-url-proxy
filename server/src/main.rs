/*
 ** Copyright (C) 2020-2023 KunoiSayami
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
use std::collections::HashMap;
use base64::Engine;

fn main() {
    let mut config = configparser::ini::Ini::new();
    config.load("config.ini").unwrap_or(HashMap::new());

    let bind_address = config.get("server", "addr")
        .unwrap_or(String::from("0.0.0.0"));

    let bind_port = config.getint("server", "port")
        .unwrap_or(Some(7878)).unwrap_or(7878);

    let listener = TcpListener::bind(format!("{}:{}", bind_address, bind_port)).unwrap();

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
    let website_address = String::from_utf8(base64::engine::general_purpose::STANDARD.decode(tmp.last().unwrap()).unwrap()).unwrap();

    match webbrowser::open(website_address.as_str()) {
        Ok(_) => println!("Opened {}", website_address),
        Err(e) => eprintln!("Got error {:?}", e)
    }

    let response = format!(
        "HTTP/1.1 204 No Content\r\nContent-Type: text/html"
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}