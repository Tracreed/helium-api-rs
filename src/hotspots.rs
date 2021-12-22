use crate::{models::{Reward, QueryTimeRange, Hotspot}, *};

/// Get all known hotspots
pub fn all(client: &Client) -> Stream<Hotspot> {
    client.fetch_stream("/hotspots", NO_QUERY)
}

/// Get a specific hotspot by its address
pub async fn get(client: &Client, address: &str) -> Result<Hotspot> {
    client
        .fetch(&format!("/hotspots/{}", address), NO_QUERY)
        .await
}

/// Get rewards for a hotspot
///
/// Returns rewards for a given validator per reward block the validator is in,
/// for a given timeframe. `QueryTimeRange` contains the timestamps given in
/// 4ISO 8601 format, or in relative time. The block that contains the max_time
/// timestamp is excluded from the result.
pub fn rewards(client: &Client, address: &str, query: &QueryTimeRange) -> Stream<Reward> {
    client.fetch_stream(&format!("/hotspots/{}/rewards", address), query)
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
        let hotspot = hotspots::get(
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
