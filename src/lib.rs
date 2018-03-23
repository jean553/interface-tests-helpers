//! Tests helpers to test HTTP interfaces.

extern crate reqwest;

use reqwest::{
    Client,
    Response,
    StatusCode,
};

use reqwest::header::ContentType;

use std::collections::HashMap;

pub trait HasBaseUrl {

    fn get_base_url(&self) -> &str;
}

pub trait ClientHandler {

    fn get_url(&self, url: &str) -> Response;

    fn post_json(&self, url: &str, json: &HashMap<&str, &str>) -> Response;

    fn post_body(&self, url: &str, body: &str) -> Response;

    fn put_xml(&self, url: &str, body: &str) -> Response;
}

pub trait ResponseHandler {

    fn assert_200(&self);

    fn assert_201(&self);

    fn assert_204(&self);

    fn assert_400(&self);

    fn assert_404(&self);

    fn assert_409(&self);

    fn assert_500(&self);
}

impl ClientHandler for Client {

    /// Perform a GET request without parameter
    ///
    /// # Args:
    ///
    /// `url` - the suffix of the URL
    fn get_url(
        &self,
        url: &str,
    ) -> Response {

        self.get(url)
            .send()
            .unwrap()
    }

    /// Perform a POST request to send JSON and stores its result
    ///
    /// # Args:
    ///
    /// `url` - the suffix of the URL
    /// `json` - the json data to send
    fn post_json(
        &self,
        url: &str,
        json: &HashMap<&str, &str>,
    ) -> Response {

        self.post(url)
            .json(json)
            .send()
            .unwrap()
    }

    /// Perform a POST request to send a raw body and stores its result
    ///
    /// # Args:
    ///
    /// `url` - the suffix of the URL
    /// `body` - the body data to send (String format)
    fn post_body(
        &self,
        url: &str,
        body: &str,
    ) -> Response {

        self.post(url)
            .body(body.to_string())
            .header(ContentType::plaintext())
            .send()
            .unwrap()
    }

    /// Perform a PUT request to send a XML body and returns it result
    ///
    /// # Args:
    ///
    /// `url` - the suffix of the URL
    /// `body` - the body data to send (XML format)
    fn put_xml(
        &self,
        url: &str,
        body: &str,
    ) -> Response {

        self.put(url)
            .body(body.to_string())
            .header(ContentType::xml())
            .send()
            .unwrap()
    }
}

impl ResponseHandler for Response {

    /// Assertion that checks the response status code is 200
    fn assert_200(&self) {

        assert_eq!(
            self.status(),
            StatusCode::Ok,
        );
    }

    /// Assertion that checks the response status code is 201
    fn assert_201(&self) {

        assert_eq!(
            self.status(),
            StatusCode::Created,
        );
    }

    /// Assertion that checks the response status code is 204
    fn assert_204(&self) {

        assert_eq!(
            self.status(),
            StatusCode::NoContent,
        );
    }

    /// Assertion that checks the response status code is 400
    fn assert_400(&self) {

        assert_eq!(
            self.status(),
            StatusCode::BadRequest,
        );
    }

    /// Assertion that checks the response status code is 404
    fn assert_404(&self) {

        assert_eq!(
            self.status(),
            StatusCode::NotFound,
        );
    }

    /// Assertion that checks the response status code is 409
    fn assert_409(&self) {

        assert_eq!(
            self.status(),
            StatusCode::Conflict,
        );
    }

    /// Assertion that checks the response status code is 500
    fn assert_500(&self) {

        assert_eq!(
            self.status(),
            StatusCode::InternalServerError,
        );
    }
}

#[cfg(test)]
mod tests;
