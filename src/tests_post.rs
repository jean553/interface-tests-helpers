extern crate mockito;

#[cfg(test)]
mod tests {

    use tests_post::mockito::mock;

    use lib::ClientTest;

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

        let mut client = ClientTest::new();

        client.post_json(
            API,
            &json,
        );

        client.assert_201();
    }
}
