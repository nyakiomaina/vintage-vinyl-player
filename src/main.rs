mod auth;
mod playlist;
mod player;
mod models;

use yew::prelude::*;
use wasm_bindgen::prelude::*;

use auth::Auth;
use playlist::PlaylistComponent;
use player::Player;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Auth>
            <header class="app-header">
                <h1>{ "Vintage Vinyl Player" }</h1>
            </header>
            <div>
                <Player />
                <PlaylistComponent />
            </div>
        </Auth>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {

    gloo_console::log!("Starting app...");
    yew::Renderer::<App>::new().render();
    gloo_console::log!("App started!");
}

fn main() {
    println!("Running in CLI mode (not the browser).");
}
