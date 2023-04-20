use std::sync::Arc;

use anyhow::{anyhow, Result};
use log::info;
use regex::Regex;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};

use crate::random_user_agent;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatgptParams {
    pub prompt: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u16>,
    pub top_p: Option<u8>,
    pub frequency_penalty: Option<u8>,
    pub presence_penalty: Option<u8>,
    pub stop_sequences: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Chatgpt {
    client: Client,
    base_url: &'static str,
}

fn is_complete(str: &str) -> bool {
    str.ends_with("```") || Regex::new(r"[。？！.!?]$").unwrap().is_match(str)
}

impl Chatgpt {
    pub fn new() -> Result<Self> {
        let base_url = "https://play.vercel.ai/";
        let mut headers = HeaderMap::new();
        headers.append("origin", HeaderValue::from_str(base_url)?);
        headers.append("referer", HeaderValue::from_str(base_url)?);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .http2_prior_knowledge()
            .build()?;

        Ok(Self { client, base_url })
    }

    pub async fn request(self: Arc<Self>, chatgpt_params: &ChatgptParams) -> Result<String> {
        let params = serde_json::json!(chatgpt_params);
        let url = format!("{}api/generate", self.base_url);
        let ua = random_user_agent();
        info!("random user agent:{:?}", ua);
        let response = self
            .client
            .post(url)
            .json(&params)
            .header("user-agent", HeaderValue::from_str(&ua)?)
            .send()
            .await?
            .text()
            .await?;
        let str = response
            .split('\n')
            .map(|c| match c.len() > 2 {
                true => &c[1..c.len() - 1],
                false => "",
            })
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>()
            .join("")
            .replace("\\n", "\n")
            .replace("\\\"", "\"");
        Ok(str)
    }

    pub async fn ask(self: Arc<Self>, chatgpt_params: ChatgptParams) -> Result<String> {
        let mut res = self.clone().request(&chatgpt_params).await?;
        for _ in 0..10 {
            if res.contains("limit exceede") {
                return Err(anyhow!("Rate limit exceeded!"));
            }
            if is_complete(&res) {
                break;
            }
            let mut params = chatgpt_params.clone();
            params.prompt = params.prompt + &res;
            let that = self.clone();
            let tmp = that.request(&params).await?;
            res += &tmp;
        }
        Ok(res)
    }
}
