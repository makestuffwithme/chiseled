use reqwest::Client;
use serde_json::Value;

use crate::model::trade_query::TradeQuery;
use crate::model::trade_result::TradeResult;

pub async fn fetch_mappings(client: &Client) -> Result<(Value, Value), reqwest::Error> {
    async fn fetch_stats(client: &Client) -> Result<Value, reqwest::Error> {
        client
            .get("https://www.pathofexile.com/api/trade2/data/stats")
            .send()
            .await?
            .json()
            .await
    }

    async fn fetch_items(client: &Client) -> Result<Value, reqwest::Error> {
        client
            .get("https://www.pathofexile.com/api/trade2/data/items")
            .send()
            .await?
            .json()
            .await
    }

    tokio::try_join!(fetch_stats(client), fetch_items(client))
}

pub async fn search_trade(client: &Client, query: &TradeQuery) -> Result<String, String> {
    // Search for items
    let response = client
        .post("https://www.pathofexile.com/api/trade2/search/Standard")
        .json(&query)
        .send()
        .await
        .map_err(|e| format!("Search request failed: {}", e))?;

    log::info!("Search query: {}", &query.query);

    let search_text = response.text().await.map_err(|e| e.to_string())?;
    if !search_text.starts_with('{') {
        return Err(format!("Invalid search response: {}", search_text));
    }

    // Parse the search response
    let search_json: Value = serde_json::from_str(&search_text)
        .map_err(|e| format!("Failed to parse search response: {}", e))?;

    log::info!("Search JSON: {}", search_json);

    // Extract the first 10 result IDs
    let result_ids = search_json["result"]
        .as_array()
        .ok_or("No results found")?
        .iter()
        .take(10)
        .map(|v| v.as_str().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(",");

    if result_ids.is_empty() {
        return Err("No results found".to_string());
    }

    log::info!("Result IDs: {}", result_ids);

    // Fetch item details
    let fetch_response = client
        .get(format!(
            "https://www.pathofexile.com/api/trade2/fetch/{}",
            result_ids
        ))
        .send()
        .await
        .map_err(|e| format!("Fetch request failed: {}", e))?;

    let fetch_text = fetch_response.text().await.map_err(|e| e.to_string())?;
    if !fetch_text.starts_with('{') {
        return Err(format!("Invalid fetch response: {}", fetch_text));
    }

    // Parse into our TradeResult struct
    let trade_result: TradeResult = match serde_json::from_str(&fetch_text) {
        Ok(result) => result,
        Err(e) => {
            log::error!("Failed to parse fetch response: {}", e);
            log::error!("Raw response: {}", fetch_text);
            return Err(format!(
                "Failed to parse fetch response into TradeResult: {}\nRaw response: {}",
                e, fetch_text
            ));
        }
    };

    // Convert back to string to return
    let result_str = serde_json::to_string(&trade_result)
        .map_err(|e| format!("Failed to serialize TradeResult: {}", e))?;

    log::info!("Final result: {}", result_str);
    Ok(result_str)
}
