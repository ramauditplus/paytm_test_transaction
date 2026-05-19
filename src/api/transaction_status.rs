use axum::{Json, http::StatusCode, response::IntoResponse};
use paytm_checksum::PaytmChecksum;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::env;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaytmTransactionStatusRequest {
    order_id: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    success: bool,
    data: Value,
}

pub async fn transaction_status(
    Json(req): Json<PaytmTransactionStatusRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let key = env::var("PAYTM_MERCHANT_KEY").unwrap();
    let base_url = env::var("BASE_URL").unwrap();
    let mid = env::var("PAYTM_MERCHANT_ID").unwrap();

    let mut params = BTreeMap::new();
    params.insert("mid".to_string(), mid);
    params.insert("orderId".to_string(), req.order_id);

    println!("Params: {:#?}", params);

    let checksum = PaytmChecksum::generate_signature(&params, &key);

    let client = Client::new();

    let paytm_params = json!({
        "body": params,
        "head": {
            "signature": checksum.unwrap(),
        }
    });

    println!("Paytm Params: {:#?}", paytm_params);

    let response = client
        .post(format!("{}/v3/order/status", base_url))
        .json(&paytm_params)
        .send()
        .await
        .unwrap();

    let response_json: Value = response
        .json()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    Ok(Json(ApiResponse {
        success: true,
        data: response_json,
    }))
}