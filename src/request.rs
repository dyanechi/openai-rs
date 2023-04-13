use reqwest::StatusCode;

use crate::openai::OpenAi;
use crate::*;

// type RqWestResponse = Result<reqwest::blocking::Response, reqwest::Error>;

pub trait ApiRequest {
    // fn req(&self, uri: &str, method: ApiMethod) -> ApiResult<Json>;
    fn post(&self, url: &str, body: Json) -> ApiResult<Json>;
    fn get(&self, url: &str) -> ApiResult<Json>;
}

pub enum ApiMethod {
    Get,
    Post(Json)
}

impl ApiRequest for OpenAi {
    fn get(&self, uri: &str) -> ApiResult<Json> {
        self.req(uri, ApiMethod::Get)
    }
    fn post(&self, uri: &str, body: Json) -> ApiResult<Json> {
        self.req(uri, ApiMethod::Post(body))
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct OpenAiError {
    code: Option<String>,
    message: String,
    param: Option<String>,
    _type: Option<String>,
}


impl OpenAi {
    fn req(&self, uri: &str, method: ApiMethod) -> ApiResult<Json> {
        let request = match method {
            ApiMethod::Get => self.agent.get(self.api_url.clone() + uri),
            ApiMethod::Post(body) => self.agent.post(self.api_url.clone() + uri).body(body.to_string()),
        }
        .header("Content-Type", "application/json")
        .header("OpenAI-Organization", &self.auth.organization.clone().unwrap_or_default())
        .header("Authorization", &format!("Bearer {}", self.auth.api_key))
        .build()
        .expect("should build request");

        // println!("{:#?}", request);

        match self.agent.execute(request) {
            Ok(r) => {
                let status = r.status().clone();
                if status.is_success() {
                    return Ok(r.json().unwrap())
                } else {
                    let OpenAiError { message, .. } = r.json::<JsonErr<OpenAiError>>().unwrap().error.unwrap_or_default();
                    return Err(Error::RequestError(format!("[{}]: {}", status, message)))
                }
            },
            Err(e) => {
                let err_msg = e.to_string();
                return Err(Error::RequestError(err_msg))
            }
        }
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