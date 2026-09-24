use std::{marker::PhantomData, pin::Pin, sync::Arc};

use reqwest::{
    Body, Method, RequestBuilder, StatusCode,
    header::{self, HeaderValue, InvalidHeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::queue::PluralKitQueue;

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

impl<T: DeserializeOwned> Request<T> {
    async fn send(self) -> Result<Response<T>, crate::Error> {
        let response = self
            .queue
            .push(self.builder, self.priority)
            .await??
            .error_for_status()?;

        Ok(Response {
            response,
            marker: PhantomData,
        })
    }
}

impl<T: DeserializeOwned + 'static> IntoFuture for Request<T> {
    type Output = Result<Response<T>, crate::Error>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Response<T>, crate::Error>>>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

impl Request<EmptyBody> {
    async fn send(self) -> Result<Response<EmptyBody>, crate::Error> {
        let response = self
            .queue
            .push(self.builder, self.priority)
            .await??
            .error_for_status()?;

        if response.status() != StatusCode::NO_CONTENT {
            return Err(format!(
                "expected status code 204 but received {}",
                response.status().as_u16(),
            )
            .into());
        }

        Ok(response.into())
    }
}

impl IntoFuture for Request<EmptyBody> {
    type Output = Result<Response<EmptyBody>, crate::Error>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Response<EmptyBody>, crate::Error>>>>;

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
