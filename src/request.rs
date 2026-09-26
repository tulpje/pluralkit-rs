use std::{marker::PhantomData, pin::Pin, sync::Arc};

use reqwest::{
    Body, Method, RequestBuilder, StatusCode,
    header::{self, HeaderValue, InvalidHeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    models::error::{self, ErrorResponse},
    queue::PluralKitQueue,
};

const DEFAULT_PRIORITY: u16 = 50;

pub struct EmptyBody;

pub struct Request<T: Send + Sync> {
    pub(crate) builder: RequestBuilder,
    pub(crate) queue: Arc<PluralKitQueue>,
    pub(crate) priority: u16,

    marker: PhantomData<T>,
}

impl<T: Send + Sync> Request<T> {
    pub(crate) fn new(
        client: &reqwest::Client,
        method: Method,
        base_url: &str,
        path: &str,
        queue: Arc<PluralKitQueue>,
    ) -> Self {
        Self {
            builder: client.request(method, format!("{base_url}{path}")),
            queue,
            priority: DEFAULT_PRIORITY,
            marker: PhantomData,
        }
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.builder = self.builder.query(&[&key.into(), &value.into()]);
        self
    }

    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.builder = self.builder.body(body);
        self
    }

    pub fn json<B: Serialize>(mut self, json: &B) -> Self {
        self.builder = self.builder.json(json);
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Result<Self, InvalidHeaderValue> {
        self.builder = self
            .builder
            .header(header::AUTHORIZATION, HeaderValue::from_str(&token.into())?);

        Ok(self)
    }

    pub fn with_priority(mut self, priority: u16) -> Self {
        self.priority = priority;
        self
    }
}

impl<T: DeserializeOwned + Send + Sync> Request<T> {
    async fn send(self) -> Result<Response<T>, error::PluralKitError> {
        let response = self
            .queue
            .push(self.builder, self.priority)
            .await
            .map_err(|err| error::PluralKitError::Other(err.into()))?
            .map_err(error::PluralKitError::Reqwest)?;

        if !response.status().is_success() {
            return Err(handle_error(response).await);
        }

        Ok(Response {
            response,
            marker: PhantomData,
        })
    }
}

impl<T: DeserializeOwned + Send + Sync + 'static> IntoFuture for Request<T> {
    type Output = Result<Response<T>, error::PluralKitError>;
    type IntoFuture =
        Pin<Box<dyn Future<Output = Result<Response<T>, error::PluralKitError>> + Send + Sync>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

impl Request<EmptyBody> {
    async fn send(self) -> Result<Response<EmptyBody>, error::PluralKitError> {
        let response = self
            .queue
            .push(self.builder, self.priority)
            .await
            .map_err(|err| error::PluralKitError::Other(err.into()))?
            .map_err(error::PluralKitError::Reqwest)?;

        if !response.status().is_success() {
            return Err(handle_error(response).await);
        }

        if response.status() != StatusCode::NO_CONTENT {
            return Err(error::PluralKitError::Other(
                format!(
                    "expected status code 204 but received {}",
                    response.status().as_u16(),
                )
                .into(),
            ));
        }

        Ok(response.into())
    }
}

impl IntoFuture for Request<EmptyBody> {
    type Output = Result<Response<EmptyBody>, error::PluralKitError>;
    type IntoFuture = Pin<
        Box<dyn Future<Output = Result<Response<EmptyBody>, error::PluralKitError>> + Send + Sync>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

pub struct Response<T> {
    response: reqwest::Response,
    marker: PhantomData<T>,
}

impl<T> Response<T> {
    pub fn response(&self) -> &reqwest::Response {
        &self.response
    }
}

impl<T> From<reqwest::Response> for Response<T> {
    fn from(response: reqwest::Response) -> Self {
        Response {
            response,
            marker: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> Response<T> {
    pub async fn model(self) -> Result<T, reqwest::Error> {
        self.response.json().await
    }
}
async fn handle_error(response: reqwest::Response) -> error::PluralKitError {
    let status = response.status();
    let content = response.text().await;

    if let Ok(Ok(error_response)) = content
        .as_ref()
        .map(|content| serde_json::from_str::<ErrorResponse>(content))
    {
        error::PluralKitError::PluralKit {
            code: error_response.code,
            status: status.as_u16(),
            error: error_response,
        }
    } else {
        error::PluralKitError::Http {
            status: status.as_u16(),
            text: content.ok(),
        }
    }
}
