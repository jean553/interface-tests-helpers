# rust-interface-tests-helper

Routines for HTTP interface testing in Rust.

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

use rust_interface_tests_helper::ClientTest;

#[test]
fn test() {

    let json = HashMap::new();
    json.insert("key", "value");

    let mut client = ClientTest::new();
    client.post_json("/api/1/resource", &json);

    client.assert_200();
}
```
