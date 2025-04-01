use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct RedditResponse {
    data: RedditData,
}

#[derive(Debug, Deserialize)]
struct RedditData {
    children: Vec<RedditChild>,
    after: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RedditChild {
    data: SavedItem,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SavedItem {
    title: Option<String>,
    permalink: Option<String>,
    url: Option<String>,
    subreddit: String,
    created_utc: f64,
    id: String,
    name: String,
    #[serde(rename = "is_self")]
    is_self_post: Option<bool>,
    body: Option<String>,
    link_title: Option<String>,
    link_permalink: Option<String>,
}

pub struct FetchRedditSavedData {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    client: reqwest::Client,
}

impl FetchRedditSavedData {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let client_id = env::var("REDDIT_CLIENT_ID")?;
        let client_secret = env::var("REDDIT_CLIENT_SECRET")?;
        let username = env::var("REDDIT_USERNAME")?;
        let password = env::var("REDDIT_PASSWORD")?;

        Ok(FetchRedditSavedData {
            client_id,
            client_secret,
            username,
            password,
            client: reqwest::Client::new(),
        })
    }

    pub async fn authenticate(&self) -> Result<String, Box<dyn Error>> {
        let params = [
            ("grant_type", "password"),
            ("username", &self.username),
            ("password", &self.password),
        ];

        // TODO: Dynamically get the current version of taskmaster
        let auth_response = self
            .client
            .post("https://www.reddit.com/api/v1/access_token")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&params)
            .header(USER_AGENT, "taskmaster/0.2")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let access_token = auth_response["access_token"]
            .as_str()
            .ok_or("No access token in response")?
            .to_string();

        Ok(access_token)
    }

    pub async fn fetch_saved_items(
        &self,
        access_token: &str,
    ) -> Result<Vec<SavedItem>, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("taskmaster/0.2"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", access_token))?,
        );

        let mut all_saved_items = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let url = format!(
                "https://oauth.reddit.com/user/{}/saved?limit=100{}",
                self.username,
                match &after {
                    Some(after_value) => format!("&after={}", after_value),
                    None => String::new(),
                }
            );

            let response = self
                .client
                .get(&url)
                .headers(headers.clone())
                .send()
                .await?
                .json::<RedditResponse>()
                .await?;

            for child in response.data.children {
                all_saved_items.push(child.data);
            }

            after = response.data.after;

            if after.is_none() || all_saved_items.len() >= 1000 {
                break;
            }
        }

        Ok(all_saved_items)
    }

    pub fn save_to_file(items: &Vec<SavedItem>, filename: &str) -> Result<(), Box<dyn Error>> {
        let permalinks: Vec<Option<String>> =
            items.iter().map(|item| item.permalink.clone()).collect();

        let json = serde_json::to_string_pretty(&permalinks)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub async fn fetch_and_save(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let access_token = self.authenticate().await?;
        let saved_items = self.fetch_saved_items(&access_token).await?;

        println!("Fetched {} saved items", saved_items.len());
        Self::save_to_file(&saved_items, filename)?;
        println!("Saved items to {}", filename);

        Ok(())
    }
}
