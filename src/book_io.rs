use crate::rest_api::get;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub data: Vec<Collection>,
    #[serde(rename = "type")]
    pub response_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Collection {
    pub collection_id: String,
    pub description: String,
}

impl Collection {
    /// Validates that a policy is also a Book.io collection
    /// * `policy_id` - A policy id, as string, to validate.
    /// If the policy_id is a valid Book.io collection, return the collection.
    pub async fn validate(policy_id: &str) -> Result<Collection, String> {
        // I couldn't find any documentation on this.  This call would be much more efficient
        // if there were a path param or query string that accepts a collection id.
        let resp = get::<Response>("https://api.book.io/api/v0/collections", None)
            .await
            .expect("HTTP Error");
        // case sensitivity and leading / trailing whitespace could be a concern here
        let validated = resp.data.iter().find(|c| c.collection_id == policy_id);
        if let Some(coll) = validated {
            Ok(coll.to_owned())
        } else {
            Err("Book.io collection not found".parse().unwrap())
        }
    }
}
