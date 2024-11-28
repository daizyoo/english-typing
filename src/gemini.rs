#![allow(warnings)]

use std::env;

use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Gemini API
///
/// # Example
/// ```
/// let gemini = Gemini::default().ask("こんにちは".to_string()).await.unwrap();
/// println!("{}", gemini.text());
///
/// let gemini = Gemini::new(YOU_API_KEY, gemini-1.5-flash).ask("Hello".to_string()).await.unwrap();
/// println!("{}", gemini.text());
/// ```
pub struct Gemini {
    api_key: String,
    model: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Text {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Parts {
    parts: Vec<Text>,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Parts>,
}

impl Default for Gemini {
    fn default() -> Self {
        Self {
            api_key: env::var("GEMINI_API_KEY").unwrap(),
            model: String::from("gemini-1.5-flash-latest"),
        }
    }
}

impl Gemini {
    fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
    pub async fn ask(&self, prompt: String) -> anyhow::Result<GeminiResponse> {
        let client = Client::new();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );
        let body = GeminiRequest {
            contents: vec![Parts {
                parts: vec![Text { text: prompt }],
            }],
        };
        let response = client.post(url).json(&body).send().await?;

        Ok(response.json().await?)
    }
}

impl GeminiResponse {
    pub fn text(&self) -> String {
        self.candidates[0].content.parts[0].text.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    candidates: Vec<Candidate>,
    modelVersion: String,
    usageMetadata: UsageMetadata,
}

#[derive(Debug, Deserialize)]
struct UsageMetadata {
    candidatesTokenCount: u64,
    promptTokenCount: u64,
    totalTokenCount: u64,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<Parts>,
    role: String,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    avgLogprobs: f64,
    content: Parts,
    finishReason: String,
}
