use yew::prelude::*;
use gloo_storage::{Storage, LocalStorage};
use web_sys::window;
use gloo_console::log;

const STORAGE_KEY: &str = "spotify_token";

#[derive(Properties, PartialEq)]
pub struct AuthProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct Auth {
    token: Option<String>,
}

pub enum Msg {
    SetToken(String),
    NoOp,
}

impl Component for Auth {
    type Message = Msg;
    type Properties = AuthProps;

    fn create(ctx: &Context<Self>) -> Self {
        let token = LocalStorage::get::<String>(STORAGE_KEY).ok();

        if token.is_none() {
            let window = window().expect("no global `window`");
            let location = window.location();
            let hash = location.hash().unwrap_or_default();

            if hash.contains("access_token") {
                let params: Vec<&str> = hash.trim_start_matches('#').split('&').collect();
                for param in params {
                    let kv: Vec<&str> = param.split('=').collect();
                    if kv.len() == 2 && kv[0] == "access_token" {
                        let found_token = kv[1].to_string();
                        LocalStorage::set(STORAGE_KEY, &found_token).expect("Failed to set token");
                        ctx.link().send_message(Msg::SetToken(found_token));
                        let _ = location.set_hash("");
                        break;
                    }
                }
            }
        }

        Self { token }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetToken(t) => {
                self.token = Some(t);
                true
            }
            Msg::NoOp => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.token.is_some() {
            log!("Token found in Auth");
            html! {
                <>
                    { for ctx.props().children.iter() }
                </>
            }
        } else {
            log!("No token found in Auth");
            let client_id = "you have to add your own client id";
            let redirect_uri = "http://localhost:3000/";
            let scopes = "user-read-playback-state user-modify-playback-state playlist-read-private";

            let auth_url = format!(
                "https://accounts.spotify.com/authorize?response_type=token&client_id={}&scope={}&redirect_uri={}",
                client_id, scopes, redirect_uri
            );

            html! {
                <div class="auth">
                    <h1>{ "Vintage Vinyl Player" }</h1>
                    <a href={auth_url}>
                        <button>{ "Login with Spotify" }</button>
                    </a>
                </div>
            }
        }
    }
}
