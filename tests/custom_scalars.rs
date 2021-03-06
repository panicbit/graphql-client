#[macro_use]
extern crate graphql_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::net::Ipv4Addr;

// Important! The NetworkAddress scalar should deserialize to an Ipv4Addr from the Rust std library.
type NetworkAddress = Ipv4Addr;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "tests/custom_scalars/query.graphql",
    schema_path = "tests/custom_scalars/schema.graphql"
)]
pub struct CustomScalarsQuery;

#[test]
fn custom_scalars() {
    let valid_response = json!({
        "address": "127.0.1.2",
    });

    let valid_addr =
        serde_json::from_value::<custom_scalars_query::ResponseData>(valid_response).unwrap();

    assert_eq!(
        valid_addr.address.unwrap(),
        "127.0.1.2".parse::<Ipv4Addr>().unwrap()
    );

    let invalid_response = json!({
        "address": "localhost",
    });

    assert!(
        serde_json::from_value::<custom_scalars_query::ResponseData>(invalid_response).is_err()
    );
}
