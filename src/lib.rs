//! Tests helpers to test HTTP interfaces.

extern crate reqwest;

use reqwest::{
    Client,
    Response,
    StatusCode,
};

use std::collections::HashMap;

pub trait HasBaseUrl {

    fn get_base_url(&self) -> &str;
}

pub trait ClientHandler {

    fn get_url(&self, url: &str) -> Response;

    fn post_json(&self, url: &str, json: &HashMap<&str, &str>) -> Response;

    fn post_body(&self, url: &str, body: &str) -> Response;

    fn put_xml(&self, url: &str, body: &str) -> Response;

    fn put_text(&self, url: &str, text: &str) -> Response;
}

pub trait ResponseHandler {

    fn assert_200(&self);

    fn assert_201(&self);

    fn assert_204(&self);

    fn assert_400(&self);

    fn assert_401(&self);

    fn assert_403(&self);

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
            .header(
                reqwest::header::CONTENT_TYPE,
                "text/plain"
            )
            .send()
            .unwrap()
    }

    /// Perform a PUT request to send a XML body and returns its result
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
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/xml"
            )
            .send()
            .unwrap()
    }

    /// Perform a PUT request to send a text body and returns its result
    ///
    /// # Args:
    ///
    /// `url` - the suffix of the URL
    /// `text` - the text to upload
    fn put_text(
        &self,
        url: &str,
        text: &str,
    ) -> Response {

        self.put(url)
            .body(text.to_string())
            .header(
                reqwest::header::CONTENT_TYPE,
                "text/plain"
            )
            .send()
            .unwrap()
    }
}

impl ResponseHandler for Response {

    /// Assertion that checks the response status code is 200
    fn assert_200(&self) {

        assert_eq!(
            self.status(),
            StatusCode::OK,
        );
    }

    /// Assertion that checks the response status code is 201
    fn assert_201(&self) {

        assert_eq!(
            self.status(),
            StatusCode::CREATED,
        );
    }

    /// Assertion that checks the response status code is 204
    fn assert_204(&self) {

        assert_eq!(
            self.status(),
            StatusCode::NO_CONTENT,
        );
    }

    /// Assertion that checks the response status code is 400
    fn assert_400(&self) {

        assert_eq!(
            self.status(),
            StatusCode::BAD_REQUEST,
        );
    }

    /// Assertion that checks the response status code is 401
    fn assert_401(&self) {

        assert_eq!(
            self.status(),
            StatusCode::UNAUTHORIZED,
        );
    }

    /// Assertion that checks the response status code is 403
    fn assert_403(&self) {

        assert_eq!(
            self.status(),
            StatusCode::FORBIDDEN,
        );
    }

    /// Assertion that checks the response status code is 404
    fn assert_404(&self) {

        assert_eq!(
            self.status(),
            StatusCode::NOT_FOUND,
        );
    }

    /// Assertion that checks the response status code is 409
    fn assert_409(&self) {

        assert_eq!(
            self.status(),
            StatusCode::CONFLICT,
        );
    }

    /// Assertion that checks the response status code is 500
    fn assert_500(&self) {

        assert_eq!(
            self.status(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    }
}

#[cfg(test)]
mod tests;
