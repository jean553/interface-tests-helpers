//! Tests helpers to test HTTP interfaces.

extern crate reqwest;

use reqwest::{
    Client,
    StatusCode,
};

use std::collections::HashMap;

const SERVICE_URL: &str = "http://localhost:8000";

/// HTTP client wrapper for tests.
struct ClientTest {
    client: Client,
    status_code: StatusCode,
}

impl ClientTest {

    /// Initializes the tests client (dummy values for the response attributes)
    ///
    /// # Returns:
    ///
    /// the tests client to use
    ///
    /// # Example:
    ///
    /// ```
    /// let mut client = ClientTest::new();
    /// ```
    pub fn new() -> ClientTest {
        ClientTest {
            client: Client::new(),

            /* no Option<T> used here, would be None
               only until the first request */
            status_code: StatusCode::Ok,
        }
    }

    /// Perform a POST request to send JSON and stores its result
    ///
    /// # Args:
    ///
    /// `url` - the suffix of the URL
    /// `json` - the json data to send
    ///
    /// # Example:
    ///
    /// ```
    /// let mut json = HashMap::new();
    /// json.insert("first_key", "first_item");
    /// json.insert("second_key", "second_item");
    ///
    /// client.post_json(
    ///     "/api/1/resources",
    ///     &json,
    /// );
    /// ```
    pub fn post_json(
        &mut self,
        url: &str,
        json: &HashMap<&str, &str>,
    ) {
        let url = format!(
            "{}{}",
            SERVICE_URL,
            url,
        );

        let response = self.client.post(&url)
            .json(json)
            .send()
            .unwrap();

        self.status_code = response.status();
    }

    /// Assertion that checks the response status code is 200
    ///
    /// # Example:
    ///
    /// ```
    /// client.assert_200();
    /// ```
    pub fn assert_200(&self) {

        assert_eq!(
            self.status_code,
            StatusCode::Ok,
        );
    }
}
