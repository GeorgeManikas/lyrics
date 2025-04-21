use reqwest::{Client, Error};
use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]
pub struct LyricsResponse {
    pub lyrics: String
}

pub async fn get_lyrics(artist: &str, song: &str) -> Result<LyricsResponse,Error> {
    let response = Client::new()
        .get(format!("https://api.lyrics.ovh/v1/{artist}/{song}"))
        .send()
        .await;

    match response {
        Ok(response) => {
            response.json::<LyricsResponse>().await
        },
        Err(error) => {
            Err(error)
        }
    }
}
