[![Build Status](https://travis-ci.org/jean553/rust-interface-tests-helper.svg?branch=master)](https://travis-ci.org/jean553/rust-interface-tests-helper)

# rust-interface-tests-helper

Routines for HTTP interface testing in Rust.

## Table of contents
- [Development](#development)
- [Generate documentation](#generate-documentation)
- [Usage](#usage)
- [Tests](#tests)

## Development

Build the development container.

```sh
vagrant up
```

Connect to the development container.

```sh
vagrant ssh
```

## Generate documentation

```sh
cargo rustdoc -- --document-private-items
```

## Usage

```rust
extern crate rust_interface_tests_helper;

#[cfg(test)]
mod tests {

    use lib::{
        ClientHandler,
        ResponseHandler,
        HasBaseUrl,
    };

    use reqwest::{
        Client,
        Response,
    };

    use std::collections::HashMap;

    impl HasBaseUrl for Client {

        /// Returns the service base URL.
        ///
        /// # Returns:
        ///
        /// the service base URL.
        fn get_base_url(&self) -> &str {
            "http://localhost:1234"
        }
    }

    trait ResourceHandler {

        fn post_resource(&self, json: &HashMap<&str, &str>) -> Response;
    }

    impl ResourceHandler for Client {

        /// Example of "per resource implementation" method.
        ///
        /// # Arguments:
        ///
        /// `json` - the json data to send
        fn post_resource(
            &self,
            json: &HashMap<&str, &str>,
        ) -> Response {

            return self.post_json(
                "/resource",
                json,
            );
        }
    }

    #[test]
    fn test_post() {

        let mut json: HashMap<&str, &str> = HashMap::new();
        json.insert("key", "value");

        let client = Client::new();
        let response = client.post_resource(&json);

        response.assert_201();
    }
}
```

## Tests

One thread only must be used (the mocked URL is updated for every test but remains the same every time).

```rust
cargo test -- --test-threads=1
```
