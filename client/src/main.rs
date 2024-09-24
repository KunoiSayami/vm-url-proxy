/*
 ** Copyright (C) 2020-2024 KunoiSayami
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
use base64::Engine;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    remote: String,
    passkey: String,
}

fn get_config() -> &'static str {
    "config.toml"
}

#[cfg(not(debug_assertions))]
fn get_config() -> std::path::PathBuf {
    let mut exec_config = std::env::current_exe()?;
    exec_config.pop();
    exec_config.push("config.toml");
    exec_config
}

async fn async_main(body: String, config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("body: {body}");
    //sleep(Duration::from_secs(10));
    let client = reqwest::Client::new();
    let data = base64::engine::general_purpose::STANDARD_NO_PAD.encode(body);
    let res = client
        .post(&config.remote)
        .json(&types::Body::new(
            auth::generate(&config.passkey, &data),
            data,
        ))
        .send()
        .await?;
    println!("response: {res:#?}");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Ok(());
    }
    let config: Config = toml::from_str(&std::fs::read_to_string(get_config())?)?;

    let body = args.get(1).unwrap().clone();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main(body, config))
}
