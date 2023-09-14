use axum::response::IntoResponse;

// Returns JSON as a response
pub async fn hello_server() -> impl IntoResponse {
    "Hello from the Server!"
}
