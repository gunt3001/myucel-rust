use reqwest::{Client, Error};
use serde::Deserialize;
use urlencoding::encode;

#[derive(Debug, Deserialize)]
pub struct RedditPost {
    pub(crate) title: String,
    pub(crate) url: String,
}

/// Myucel is a library for searching for anime discussion threads on Reddit
#[derive(Debug)]
pub struct Myucel;

impl Myucel {
    /// Search Reddit for discussion threads about a series
    pub async fn search_reddit(title: &str) -> Result<Vec<RedditPost>, Error> {
        let client = Client::new();
        let query = format!("title:({} episode discussion)", title);
        let url = format!("https://www.reddit.com/r/anime/search.json?q={}&sort=new&restrict_sr=true", encode(&query));

        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .header("User-Agent", "u-gunt3001-myucel-rust/1.0")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let posts = response["data"]["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(|post| {
                let data = &post["data"];
                RedditPost {
                    title: data["title"].as_str().unwrap().to_string(),
                    url: data["url"].as_str().unwrap().to_string(),
                }
            })
            .collect();

        Ok(posts)
    }
}