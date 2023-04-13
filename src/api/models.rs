use crate::{*, request::ApiRequest};
use super::{MODELS_LIST, MODELS_RETRIEVE};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub object: Option<String>,
    pub created: Option<usize>,
    pub owned_by: Option<String>,
    pub permission: Vec<Permission>,
    pub root: Option<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
	pub id: String,
	pub object: Option<String>,
	pub created: u64,
	pub allow_create_engine: bool,
	pub allow_sampling: bool,
	pub allow_logprobs: bool,
	pub allow_search_indices: bool,
	pub allow_view: bool,
	pub allow_fine_tuning: bool,
	pub organization: Option<String>,
	pub group: Option<String>,
	pub is_blocking: bool,
}

pub trait ModelsApi {
    fn models_list(&self) -> ApiResult<Vec<Model>>;
    fn models_retrieve(&self, model_id: &str) -> ApiResult<Model>;
}

impl ModelsApi for OpenAi {
    fn models_list(&self) -> ApiResult<Vec<Model>> {
        let res: Json = self.get(MODELS_LIST)?;
        let Some(data) = res.as_object().unwrap().get("data") else {
            return Err(Error::ApiError("No data retrieved".to_string()));
        };

        let data = serde_json::from_value(data.clone());
        if let Err(e) = data {
            return Err(Error::ApiError(e.to_string()))
        }

        Ok(data.unwrap())

        
        // let data = serde_json::from_value(data.clone()).unwrap_or_else(|| {
        //     return Err(Error::ApiError(data.err().unwrap().to_string()))
        // });

        // let Ok(data) = serde_json::from_value(data.clone()) else {
        //     return Err(Error::ApiError(data.err().unwrap().to_string()))
        // };

        
    }

    fn models_retrieve(&self, model_id: &str) -> ApiResult<Model> {
        let res: Json = self.get(&(MODELS_RETRIEVE.to_owned() + model_id))?;
        Ok(serde_json::from_value(res).unwrap())
    }
}
