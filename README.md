# Vintage Vinyl Player

Yew (Rust + WebAssembly) app that integrates with Spotify to play/pause tracks and display user playlists, all while showing a spinning vinyl UI. When a track is playing, the vinyl spins; when paused, it stops.

## Features
- Spotify OAuth - users can log in via Spotify’s OAuth flow
- Play/Pause/Next/Previous - controls playback on an active Spotify device
- Playlist gallery - displays your Spotify playlists with cover images
- Animated turntable - stylized vinyl record spins during playback and pauses otherwise.
  
## Requirements
1. Spotify Premium account (required for controlling playback from third-party apps)
2. A Spotify Developer application:
- Go to the spotify developer dashboard.
- Create a new app and add your local redirect URI (e.g. http://localhost:3000/) under “Redirect URIs.”
- Copy your client_id into the code (in auth.rs).
- Rust toolchain, wasm-bindgen, and either wasm-pack or trunk.
- A local or hosted server to serve the .wasm files (cannot open via file://)
  
## Building & Running
Using wasm-pack

Install: ```cargo install wasm-pack```

Build:
```wasm-pack build --target web --out-name vintage_vinyl_player --out-dir pkg```

Serve (for example, using Python):
python -m http.server 3000

Then open http://localhost:3000 in your browser

## Usage
- Open the app in your browser
- Click “Login with Spotify.”
- Return to the app; it will show your playlists and a vinyl turntable
- Click Play/Pause/Next/Previous icons to control the currently active Spotify device.
  
The vinyl spins while a track is playing and stops when paused.

## Troubleshooting
If no device is active in Spotify, calls to play/pause might do nothing. Ensure at least one Spotify app (phone, desktop, or web) is open and playing.
A 401 or 403 error means your token is invalid or you don’t have Spotify Premium. Check console logs.
If you see a blank page, open DevTools → Console to see if the .wasm files are loading correctly or if there’s a CORS/error message.
