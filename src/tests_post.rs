extern crate mockito;
extern crate reqwest;

#[cfg(test)]
mod tests {

    use lib::{
        ClientHandler,
        ResponseHandler,
    };

    use reqwest::Client;

    use tests_post::mockito::mock;

    use std::collections::HashMap;

    #[test]
    fn test_post() {

        const API: &str = "/resource";
        let _m = mock("POST", API)
            .with_status(201)
            .with_body("OK")
            .create();

        let mut json: HashMap<&str, &str> = HashMap::new();
        json.insert("key", "value");

        let client = Client::new();
        let response = client.post_json(API, &json);

        response.assert_201();
    }
}
