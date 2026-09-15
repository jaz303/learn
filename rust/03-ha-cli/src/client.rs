use crate::models::{Error, State};
use reqwest::blocking::Client as HttpClient;
use serde::de::DeserializeOwned;

pub struct Client {
    base_url: String,
    api_key: String,
    http_client: HttpClient,
}

impl Client {
    pub fn new(base_url: String, api_key: String) -> Self {
        Client {
            base_url,
            api_key,
            http_client: HttpClient::new(),
        }
    }

    pub fn get_components(&self) -> Result<Vec<String>, Error> {
        self.get("/components")
    }

    pub fn get_states(&self) -> Result<Vec<State>, Error> {
        self.get("/states")
    }

    pub fn set_state(&self, entity_id: &str, on: bool) -> Result<serde_json::Value, Error> {
        #[derive(serde::Serialize)]
        struct Payload<'a> {
            state: &'a str,
        }

        let payload = Payload {
            state: if on { "on" } else { "off" },
        };

        let path = format!("/states/{}", entity_id);
        self.post(&path, payload)
    }

    fn post<I: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        payload: I,
    ) -> Result<T, Error> {
        let url = format!("{}/api{}", self.base_url, path);
        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send();
        self.handle_response(response)
    }

    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let url = format!("{}/api{}", self.base_url, path);
        let response = self.http_client.get(&url).bearer_auth(&self.api_key).send();
        self.handle_response(response)
    }

    fn handle_response<T: DeserializeOwned>(
        &self,
        res: reqwest::Result<reqwest::blocking::Response>,
    ) -> Result<T, Error> {
        match res {
            Ok(r) => {
                if r.status() == 401 || r.status() == 403 {
                    Err(Error::Auth())
                } else if r.status() != 200 {
                    Err(Error::UnexpectedHttpStatus(r.status()))
                } else {
                    match r.json() {
                        Ok(v) => Ok(v),
                        Err(e) => Err(Error::Json(e)),
                    }
                }
            }
            Err(e) => Err(Error::Transport(e)),
        }
    }
}
