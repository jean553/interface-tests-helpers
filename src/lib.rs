//! Tests helpers to test HTTP interfaces.

extern crate reqwest;

mod lib {

    use reqwest::{
        Client,
        Response,
        StatusCode,
    };

    use std::collections::HashMap;

    const SERVICE_URL: &str = "http://localhost:1234";

    pub trait ClientHandler {

        fn post_json(&self, url: &str, json: &HashMap<&str, &str>) -> Response;
    }

    pub trait ResponseHandler {

        fn assert_201(&self);
    }

    impl ClientHandler for Client {

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

            let url = format!(
                "{}{}",
                SERVICE_URL,
                url,
            );

            return self.post(&url)
                .json(json)
                .send()
                .unwrap();
        }
    }

    impl ResponseHandler for Response {

        /// Assertion that checks the response status code is 201
        fn assert_201(&self) {

            assert_eq!(
                self.status(),
                StatusCode::Created,
            );
        }
    }
}

#[cfg(test)]
mod tests_post;
