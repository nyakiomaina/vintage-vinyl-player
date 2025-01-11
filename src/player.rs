use yew::prelude::*;
use gloo_storage::{Storage, LocalStorage};
use gloo_console::log;
use crate::models::CurrentPlayback;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

const STORAGE_KEY: &str = "spotify_token";

pub enum Msg {
    Play,
    Pause,
    Next,
    Previous,
    FetchPlayback,
    ReceivePlayback(Result<CurrentPlayback, String>),
    ToggleSpin,
}

pub struct Player {
    playback: Option<CurrentPlayback>,
    spinning: bool,
}

impl Component for Player {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchPlayback);
        Self {
            playback: None,
            spinning: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Play => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    if let Ok(token) = LocalStorage::get::<String>(STORAGE_KEY) {
                        let url = "https://api.spotify.com/v1/me/player/play";
                        let res = Request::put(url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;

                        match res {
                            Ok(r) if r.ok() => link.send_message(Msg::FetchPlayback),
                            Ok(r) => log!(format!("Play Error: HTTP {}", r.status())),
                            Err(_) => log!("Network error during play"),
                        }
                    }
                });
                false
            }
            Msg::Pause => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    if let Ok(token) = LocalStorage::get::<String>(STORAGE_KEY) {
                        let url = "https://api.spotify.com/v1/me/player/pause";
                        let res = Request::put(url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;

                        match res {
                            Ok(r) if r.ok() => link.send_message(Msg::FetchPlayback),
                            Ok(r) => log!(format!("Pause Error: HTTP {}", r.status())),
                            Err(_) => log!("Network error during pause"),
                        }
                    }
                });
                false
            }
            Msg::Next => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    if let Ok(token) = LocalStorage::get::<String>(STORAGE_KEY) {
                        let url = "https://api.spotify.com/v1/me/player/next";
                        let res = Request::post(url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;

                        match res {
                            Ok(r) if r.ok() => link.send_message(Msg::FetchPlayback),
                            Ok(r) => log!(format!("Next Error: HTTP {}", r.status())),
                            Err(_) => log!("Network error during next track"),
                        }
                    }
                });
                false
            }
            Msg::Previous => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    if let Ok(token) = LocalStorage::get::<String>(STORAGE_KEY) {
                        let url = "https://api.spotify.com/v1/me/player/previous";
                        let res = Request::post(url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;

                        match res {
                            Ok(r) if r.ok() => link.send_message(Msg::FetchPlayback),
                            Ok(r) => log!(format!("Previous Error: HTTP {}", r.status())),
                            Err(_) => log!("Network error during previous track"),
                        }
                    }
                });
                false
            }
            Msg::FetchPlayback => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    if let Ok(token) = LocalStorage::get::<String>(STORAGE_KEY) {
                        let url = "https://api.spotify.com/v1/me/player";
                        let res = Request::get(url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .send()
                            .await;

                        match res {
                            Ok(r) if r.ok() => {
                                match r.json::<CurrentPlayback>().await {
                                    Ok(pb) => link.send_message(Msg::ReceivePlayback(Ok(pb))),
                                    Err(_) => link.send_message(Msg::ReceivePlayback(Err(
                                        "Failed to parse CurrentPlayback JSON".into()
                                    ))),
                                }
                            }
                            Ok(r) => {
                                link.send_message(Msg::ReceivePlayback(Err(
                                    format!("Error fetching playback: HTTP {}", r.status())
                                )));
                            }
                            Err(_) => {
                                link.send_message(Msg::ReceivePlayback(Err("Network error".into())));
                            }
                        }
                    } else {
                        link.send_message(Msg::ReceivePlayback(Err("No token found".into())));
                    }
                });
                false
            }
            Msg::ReceivePlayback(result) => {
                match result {
                    Ok(pb) => self.playback = Some(pb),
                    Err(e) => {
                        log!(format!("ReceivePlayback error: {}", e));
                        self.playback = None;
                    }
                }
                self.spinning = self
                    .playback
                    .as_ref()
                    .and_then(|pb| pb.item.clone())
                    .is_some();

                true
            }
            Msg::ToggleSpin => {
                self.spinning = !self.spinning;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let playback_info = if let Some(pb) = &self.playback {
            if let Some(item) = &pb.item {
                let artists = item
                    .artists
                    .iter()
                    .map(|a| a.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Playing: {} by {}", item.name, artists)
            } else {
                "No track playing".into()
            }
        } else {
            "Loading playback info...".into()
        };

        log!("rendering player component");

        let player_class = if self.spinning {
            "vinyl-player"
        } else {
            "vinyl-player paused"
        };

        html! {
            <section class="player-section">
                <div class={player_class}
                     onclick={ctx.link().callback(|_| Msg::ToggleSpin)}>
                </div>

                <div class="playback-info">
                    <h2>{ playback_info }</h2>
                </div>

                <div class="controls">
                    <button onclick={ctx.link().callback(|_| Msg::Previous)}>
                        <svg viewBox="0 0 24 24" fill="currentColor" width="34" height="34">
                            <path d="M13 19V5l-8 7Zm7-14h-2v14h2Z"/>
                        </svg>
                    </button>

                    <button onclick={ctx.link().callback(|_| Msg::Play)}>
                        <svg viewBox="0 0 24 24" fill="currentColor" width="34" height="34">
                            <path d="M8 5v14l11-7z"/>
                        </svg>
                    </button>

                    <button onclick={ctx.link().callback(|_| Msg::Pause)}>
                        <svg viewBox="0 0 24 24" fill="currentColor" width="34" height="34">
                            <path d="M6 19h4V5H6ZM14 5v14h4V5Z"/>
                        </svg>
                    </button>

                    <button onclick={ctx.link().callback(|_| Msg::Next)}>
                        <svg viewBox="0 0 24 24" fill="currentColor" width="34" height="34">
                            <path d="M5 5v14l8-7Zm11 14h2V5h-2Z"/>
                        </svg>
                    </button>
                </div>
            </section>
        }
    }
}
