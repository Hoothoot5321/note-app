use reqwest::header;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Headers {
    pub header: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Resp {
    id: Option<i32>,
    created_at: Option<String>,
    pub header: String,
    pub content: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    pub header: String,
    pub content: String,
}

pub struct Supabase {
    base_url: String,
    custom_client: reqwest::Client,
}

impl Supabase {
    pub fn new(base_url: String, api_key: String) -> Result<Supabase, exitfailure::ExitFailure> {
        let mut acces_header = header::HeaderMap::new();
        acces_header.insert("apikey", header::HeaderValue::from_str(&api_key)?);
        acces_header.insert(
            "Content-Type",
            header::HeaderValue::from_str("application/json")?,
        );

        let custom_client = reqwest::Client::builder()
            .default_headers(acces_header)
            .build()?;

        Ok(Supabase {
            base_url,
            custom_client,
        })
    }
    pub async fn get_all(&self, table: &str) -> Result<Vec<Resp>, exitfailure::ExitFailure> {
        let resp = self
            .custom_client
            .get((&self.base_url).to_string() + "/" + table)
            .send()
            .await?;

        let data = resp.json::<Vec<Resp>>().await?;
        Ok(data)
    }
    pub async fn get_all_headers(
        &self,
        table: &str,
    ) -> Result<Vec<Headers>, exitfailure::ExitFailure> {
        let resp = self
            .custom_client
            .get((&self.base_url).to_string() + "/" + table + "?select=header")
            .send()
            .await?;

        let data = resp.json::<Vec<Headers>>().await?;
        Ok(data)
    }

    pub async fn get_from_header(
        &self,
        table: &str,
        header: &str,
    ) -> Result<Vec<Resp>, exitfailure::ExitFailure> {
        let resp = self
            .custom_client
            .get((&self.base_url).to_string() + "/" + table + "?header=eq." + header)
            .send()
            .await?;

        let data = resp.json::<Vec<Resp>>().await?;
        Ok(data)
    }

    pub async fn patch_from_header(
        &self,
        table: &str,
        header: String,
        content: String,
    ) -> Result<String, exitfailure::ExitFailure> {
        let resp = self
            .custom_client
            .patch((&self.base_url).to_string() + "/" + table + "?header=eq." + &header)
            .json(&Msg { header, content })
            .send()
            .await?;

        let data = resp.status().to_string();

        Ok(data)
    }
    pub async fn post_text(
        &self,
        table: &str,
        header: String,
        content: String,
    ) -> Result<String, exitfailure::ExitFailure> {
        let resp = self
            .custom_client
            .post((&self.base_url).to_string() + "/" + table)
            .json(&Msg { header, content })
            .send()
            .await?;
        let data = resp.status().to_string();

        Ok(data)
    }
}
