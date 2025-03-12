use anyhow::Result;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest_eventsource::{Event, EventSource};
use serde::Deserialize;
use serde_json::json;
use url::Url;

pub struct Api {
    endpoint: String,
    api_key: String,
    model: String,
}
impl Api {
    pub fn new<S1, S2, S3>(endpoint: S1, api_key: S2, model: S3) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let endpoint = endpoint.as_ref().to_owned();

        Self {
            endpoint: if endpoint.ends_with("/") {
                endpoint
            } else {
                format!("{endpoint}/")
            },
            api_key: api_key.as_ref().to_owned(),
            model: model.as_ref().to_owned(),
        }
    }

    pub async fn gen_completion<S: AsRef<str>>(
        &self,
        prompt: S,
    ) -> Result<impl StreamExt<Item = Result<Resp>>> {
        let url = Url::parse(&self.endpoint)?.join("chat/completions")?;

        let body = json!({
            "messages": [{
                "role": "user",
                "content": prompt.as_ref()
            }],
            "model": self.model,
            "max_tokens": 8192,
            "stream": true,
            "stream_options": {
                "include_usage": true
            }
        });

        let req = Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .bearer_auth(&self.api_key)
            .body(serde_json::to_string(&body)?);
        let s = EventSource::new(req)?.filter_map(async |ev| match ev {
            Ok(Event::Open) => None::<Result<Resp>>,
            Ok(Event::Message(msg)) => {
                if msg.data == "[DONE]" {
                    return None;
                }
                match serde_json::from_str::<Resp>(&msg.data) {
                    Ok(msg) => Some(Ok(msg)),
                    Err(err) => Some(Err(err.into())),
                }
            }
            Err(err) => Some(Err(err.into())),
        });

        Ok(Box::pin(s))
    }
}

#[derive(Deserialize)]
pub struct Resp {
    pub choices: Vec<RespChoice>,
    pub usage: Option<RespUsage>,
}

#[derive(Deserialize)]
pub struct RespChoice {
    pub finish_reason: Option<String>,
    pub delta: RespDelta,
}

#[derive(Deserialize)]
pub struct RespDelta {
    pub content: Option<String>,
    pub reasoning_content: Option<String>,
}

#[derive(Deserialize)]
pub struct RespUsage {
    pub completion_tokens: u64,
    pub prompt_tokens: u64,
    pub total_tokens: u64,
}
