use axum::{Json, Router};
use axum::routing::get;

pub async fn get_balance(address: String) -> Json<BalanceResponse> {
    Json(BalanceResponse { balance: 100.0 }) // TODO: Real logic
}

pub fn router() -> Router {
    Router::new()
        .route("/balance/:address", get(get_balance))
}