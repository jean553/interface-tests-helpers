//! Tests helpers to test HTTP interfaces.

extern crate reqwest;

mod lib {

    use reqwest::{
        Client,
        StatusCode,
    };

    use std::collections::HashMap;

    const SERVICE_URL: &str = "http://localhost:1234";

    /// HTTP client wrapper for tests.
    pub struct ClientTest {
        client: Client,
        status_code: StatusCode,
    }

    impl ClientTest {

        /// Initializes the tests client (dummy values for the response attributes)
        ///
        /// # Returns:
        ///
        /// the tests client to use
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

        /// Assertion that checks the response status code is 201
        pub fn assert_201(&self) {

            assert_eq!(
                self.status_code,
                StatusCode::Created,
            );
        }
    }
}

#[cfg(test)]
mod tests_get;
