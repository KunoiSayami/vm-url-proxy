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

use axum::extract::ConnectInfo;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::Router;
use base64::Engine;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use types::Body;
use webbrowser;

#[derive(Deserialize)]
struct Config {
    bind: String,
    passkey: String,
}

async fn async_main() -> anyhow::Result<()> {
    let config: Config = toml::from_str(&tokio::fs::read_to_string("config.toml").await?)?;
    if config.passkey.is_empty() {
        return Err(anyhow::anyhow!(
            "Should specify passkey or your computer may under attack"
        ));
    }

    let route = Router::new()
        .route("/", axum::routing::post(handle))
        .with_state(Arc::new(config.passkey));

    let listener = tokio::net::TcpListener::bind(&config.bind).await?;

    log::info!("Listening {}", config.bind);

    axum::serve(
        listener,
        route.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async {
        tokio::signal::ctrl_c().await.ok();
    })
    .await?;
    Ok(())
}

async fn handle(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(passkey): State<Arc<String>>,
    Json(body): Json<Body>,
) -> impl IntoResponse {
    if auth::verify(&passkey, &body.auth(), &body.auth()) {
        log::warn!("Block unauthorized request from {addr:?}, {}", &body.url());
        StatusCode::FORBIDDEN
    } else {
        if let Ok(Ok(url)) = base64::prelude::BASE64_STANDARD_NO_PAD
            .decode(body.url())
            .map(|s| String::from_utf8(s))
        {
            if let Err(e) = webbrowser::open(&url) {
                log::error!("Unable open browser: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            } else {
                log::info!("Opened {url:?}");
                StatusCode::NO_CONTENT
            }
        } else {
            StatusCode::BAD_REQUEST
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}
