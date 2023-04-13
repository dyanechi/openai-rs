use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    pub(crate) api_key: String,
    pub(crate) organization: Option<String>,
}
impl Auth {
    pub fn new(api_key: impl Into<String>, organization: Option<String>) -> Self {
        Self { api_key: api_key.into(), organization }
    }

    pub fn from_env() -> Result<Self, String> {
        let api_key = dotenvy::var("OPENAI_API_KEY").map_err(|_| "Missing OPENAI_API_KEY in env variables".to_string())?;
        let organization = match dotenvy::var("OPENAI_ORGANIZATION") {
            Ok(result) => Some(result),
            Err(_) => { eprintln!("WARN: Missing OPENAI_ORGANIZATION not specified, using default"); None }
        };
        Ok(Self { api_key, organization })
    }
}


#[derive(Debug, Clone)]
pub struct OpenAi {
    pub(crate) auth: Auth,
    pub(crate) api_url: String,
    pub(crate) agent: reqwest::blocking::Client
}
impl OpenAi {
    pub fn new(auth: Auth, api_url: String) -> Self {
        let agent = reqwest::blocking::ClientBuilder::new().build().unwrap();
        Self { auth, api_url, agent }
    }

    pub fn from_env() -> Result<Self, String> {
        let auth = Auth::from_env().unwrap();
        let api_url = dotenvy::var("OPENAI_API_URL").map_err(|_| "Missing OPENAI_API_URL in env variables".to_string())?;
        Ok(Self::new(auth, api_url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn openai_from_env() {
        let openai = OpenAi::from_env().unwrap();

        println!("{:#?}", openai);
    }

}