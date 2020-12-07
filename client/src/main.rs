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
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Ok(())
    }
    let mut config = configparser::ini::Ini::new();

    let mut exec_config = std::env::current_exe()?;
    exec_config.pop();
    exec_config.push("config.ini");

    config.load(exec_config.to_str().unwrap()).unwrap_or(HashMap::new());

    let server_address = config.get("client", "addr")
        .unwrap_or(String::from("127.0.0.1"));

    let server_port = config.getint("client", "port")
        .unwrap_or(Some(7878)).unwrap_or(7878);


    let body = args.get(1).unwrap().clone();
    println!("{}", body);
    //sleep(Duration::from_secs(10));
    let client = reqwest::blocking::Client::new();
    let res = client.post(format!("http://{}:{}/", server_address, server_port).as_str())
        .body(base64::encode(body))
        .send()?;
    println!("{:#?}", res);
    Ok(())
}
