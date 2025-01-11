use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;
use gloo_storage::{Storage, LocalStorage};
use gloo_console::log;

use crate::models::Playlist;

const STORAGE_KEY: &str = "spotify_token";

pub enum Msg {
    FetchPlaylists,
    ReceivePlaylists(Result<Vec<Playlist>, String>),
}

pub struct PlaylistComponent {
    playlists: Vec<Playlist>,
}

impl Component for PlaylistComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchPlaylists);
        Self {
            playlists: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchPlaylists => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    if let Ok(token) = LocalStorage::get::<String>(STORAGE_KEY) {
                        let url = "https://api.spotify.com/v1/me/playlists";
                        let res = Request::get(url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;

                        match res {
                            Ok(r) if r.ok() => {
                                match r.json::<serde_json::Value>().await {
                                    Ok(json_val) => {
                                        if let Some(items) = json_val.get("items").and_then(|v| v.as_array()) {
                                            let mut pls = Vec::new();
                                            for item in items {
                                                match serde_json::from_value::<Playlist>(item.clone()) {
                                                    Ok(pl) => pls.push(pl),
                                                    Err(e) => log!(format!("Failed to parse playlist: {}", e)),
                                                }
                                            }
                                            link.send_message(Msg::ReceivePlaylists(Ok(pls)));
                                        } else {
                                            link.send_message(Msg::ReceivePlaylists(
                                                Err("No 'items' found in /me/playlists".into())
                                            ));
                                        }
                                    }
                                    Err(e) => {
                                        link.send_message(Msg::ReceivePlaylists(
                                            Err(format!("Failed to parse JSON: {}", e))
                                        ));
                                    }
                                }
                            }
                            Ok(r) => {
                                link.send_message(Msg::ReceivePlaylists(
                                    Err(format!("FetchPlaylists error: HTTP {}", r.status()))
                                ));
                            }
                            Err(e) => {
                                link.send_message(Msg::ReceivePlaylists(
                                    Err(format!("Network error: {}", e))
                                ));
                            }
                        }
                    } else {
                        link.send_message(Msg::ReceivePlaylists(Err("No token found".into())));
                    }
                });
                false
            }
            Msg::ReceivePlaylists(result) => {
                match result {
                    Ok(pls) => {
                        self.playlists = pls;
                        log!(format!("Fetched {} playlists", self.playlists.len()));
                    }
                    Err(e) => {
                        log!(format!("ReceivePlaylists error: {}", e));
                        self.playlists.clear();
                    }
                }
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log!("Rendering playlists");
        html! {
            <section class="playlist-section">
                <h2>{ "Your Playlists" }</h2>
                <div class="playlist-grid">
                { for self.playlists.iter().map(|pl| {
                    let cover = pl.images.get(0)
                        .map(|img| img.url.clone())
                        .unwrap_or_else(|| "default_cover.png".to_string());

                    html! {
                        <div class="playlist-item">
                            <img src={cover} class="playlist-cover" alt="Playlist Cover" />
                            <p>{ &pl.name }</p>
                        </div>
                    }
                })}
                </div>
            </section>
        }
    }
}
