use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub images: Vec<Image>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Image {
    pub url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CurrentPlayback {
    pub item: Option<Item>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub artists: Vec<Artist>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Artist {
    pub name: String,
}
