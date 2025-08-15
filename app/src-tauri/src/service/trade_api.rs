use reqwest::Client;
use serde_json::Value;
use serde::{Deserialize, Serialize};

use crate::model::trade_query::TradeQuery;
use crate::model::trade_result::TradeResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    pub id: String,
    pub realm: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaguesResponse {
    pub result: Vec<League>,
}

async fn check_error_response(response_text: &str) -> Result<Value, String> {
    if !response_text.starts_with('{') {
        return Err(format!("Invalid JSON response: {}", response_text));
    }

    let json: Value = serde_json::from_str(response_text)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if let Some(error) = json.get("error") {
        if let Some(message) = error.get("message") {
            return Err(message.as_str().unwrap_or("Unknown error").to_string());
        }
    }

    Ok(json)
}

pub async fn fetch_mappings(client: &Client) -> Result<(Value, Value), String> {
    async fn fetch_stats(client: &Client) -> Result<Value, String> {
        let response = client
            .get("https://www.pathofexile.com/api/trade2/data/stats")
            .send()
            .await
            .map_err(|e| format!("Stats request failed: {}", e))?;

        let text = response.text().await.map_err(|e| e.to_string())?;
        check_error_response(&text).await
    }

    async fn fetch_items(client: &Client) -> Result<Value, String> {
        let response = client
            .get("https://www.pathofexile.com/api/trade2/data/items")
            .send()
            .await
            .map_err(|e| format!("Items request failed: {}", e))?;

        let text = response.text().await.map_err(|e| e.to_string())?;
        check_error_response(&text).await
    }

    tokio::try_join!(fetch_stats(client), fetch_items(client))
}

pub async fn fetch_leagues(client: &Client) -> Result<Vec<League>, String> {
    let response = client
        .get("https://www.pathofexile.com/api/trade2/data/leagues")
        .send()
        .await
        .map_err(|e| format!("Leagues request failed: {}", e))?;

    let text = response.text().await.map_err(|e| e.to_string())?;
    let json = check_error_response(&text).await?;
    
    let leagues_response: LeaguesResponse = serde_json::from_value(json)
        .map_err(|e| format!("Failed to parse leagues response: {}", e))?;
    
    Ok(leagues_response.result)
}

pub async fn search_trade(client: &Client, query: &TradeQuery, page: u32) -> Result<String, String> {
    // Search for items
    let encoded_league = urlencoding::encode(&query.league);
    let response = client
        .post(format!("https://www.pathofexile.com/api/trade2/search/{}", encoded_league))
        .json(&query)
        .send()
        .await
        .map_err(|e| format!("Search request failed: {}", e))?;

    log::info!("Search query: {}", &query.query);

    let search_text = response.text().await.map_err(|e| e.to_string())?;
    let search_json = check_error_response(&search_text).await?;

    log::info!("Search JSON: {}", search_json);

    // Calculate pagination offsets
    let per_page = 10;
    let start_idx = (page as usize - 1) * per_page;
    let total_results = search_json["result"]
        .as_array()
        .map(|arr| arr.len())
        .unwrap_or(0);

    // Extract paginated result IDs
    let result_ids = search_json["result"]
        .as_array()
        .ok_or("No results found")?
        .iter()
        .skip(start_idx)
        .take(per_page)
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
    check_error_response(&fetch_text).await?;

    // Parse into our TradeResult struct and add pagination metadata
    let mut trade_result: TradeResult = match serde_json::from_str(&fetch_text) {
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

    // Add pagination metadata to the result
    trade_result.total = total_results;
    trade_result.current_page = page as usize;
    trade_result.total_pages = (total_results + per_page - 1) / per_page;

    // Convert back to string to return
    let result_str = serde_json::to_string(&trade_result)
        .map_err(|e| format!("Failed to serialize TradeResult: {}", e))?;

    log::info!("Final result: {}", result_str);
    Ok(result_str)
}

pub async fn get_query_id(client: &Client, query: &TradeQuery) -> Result<String, String> {
    let encoded_league = urlencoding::encode(&query.league);
    let response = client
        .post(format!("https://www.pathofexile.com/api/trade2/search/{}", encoded_league))
        .json(&query)
        .send()
        .await
        .map_err(|e| format!("Search request failed: {}", e))?;

    let search_text = response.text().await.map_err(|e| e.to_string())?;
    let search_json = check_error_response(&search_text).await?;

    log::info!("Search JSON: {}", search_json);

    Ok(search_json["id"].as_str().unwrap_or_default().to_string())
}
