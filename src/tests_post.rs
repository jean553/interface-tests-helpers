extern crate mockito;
extern crate reqwest;

#[cfg(test)]
mod tests {

    use lib::{
        ClientHandler,
        ResponseHandler,
    };

    use reqwest::{
        Client,
        Response,
    };

    use tests_post::mockito::mock;

    use std::collections::HashMap;

    trait ResourceHandler {

        fn post_resource(&self, json: &HashMap<&str, &str>) -> Response;
    }

    impl ResourceHandler for Client {

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

        const API: &str = "/resource";
        let _m = mock("POST", API)
            .with_status(201)
            .with_body("OK")
            .create();

        let mut json: HashMap<&str, &str> = HashMap::new();
        json.insert("key", "value");

        let client = Client::new();
        let response = client.post_resource(&json);

        response.assert_201();
    }
}
