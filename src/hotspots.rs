use crate::{models::Hotspot, *};

/// Get all known hotspots
pub fn all(client: &Client) -> Stream<Hotspot> {
    client.fetch_stream("/hotspots", NO_QUERY)
}

/// Get a specific hotspot by its address
pub async fn get_address(client: &Client, address: &str) -> Result<Hotspot> {
    client
        .fetch(&format!("/hotspots/{}", address), NO_QUERY)
        .await
}
/// Get a specific hotspot by the 3-word animal name.
pub async fn get_name(client: &Client, name: &str) -> Result<Vec<Hotspot>> {
	client
		.fetch(&format!("/hotspots/name/{}", name), NO_QUERY)
		.await
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::default();
        let hotspots = hotspots::all(&client)
            .take(10)
            .into_vec()
            .await
            .expect("hotspots");
        assert_eq!(hotspots.len(), 10);
    }

    #[test]
    async fn get() {
        let client = Client::default();
        let hotspot = hotspots::get_address(
            &client,
            "112vvSrNAwJRSmR54aqFLEhbr6cy6T4Ufuja4VWVrxvkUAUxL2yG",
        )
        .await
        .expect("hotspot");
        assert_eq!(
            hotspot.address,
            "112vvSrNAwJRSmR54aqFLEhbr6cy6T4Ufuja4VWVrxvkUAUxL2yG"
        );
    }
}
