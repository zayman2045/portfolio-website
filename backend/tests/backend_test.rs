#[cfg(test)]
mod tests {
    use axum::{routing::post, Router, Extension};
    use axum_test::TestServer;
    use backend::routes::users::*;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use serde_json::{json, Value};
    use backend::entities::users::{self};
    use std::sync::Arc;

    // Test the create_user handler
    #[tokio::test]
    async fn test_create_user() {
        // Setup mock database
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // Mock the expected query result
            .append_query_results([[users::Model {
                id: 1,
                username: "test_user".to_owned(),
                password: "hashed_password".to_owned(),
                token: Some("test_token".to_owned()),
            }]])
            .into_connection();

        let db = Arc::new(db);

        // Create router with database extension and 
        let app = Router::new()
            .route("/users", post(create_user))
            .layer(Extension(db));

        let server = TestServer::new(app).unwrap();

        // Create test request
        let user_request: Value = json!({
            "username": "test_user",
            "password": "test_password"
        });

        // Send request to the server
        let response = server.post("/users").json(&user_request).await;

        // Validate the response
        response.assert_status_ok();
        let json_response = response.json::<UserResponse>();
        assert_eq!(json_response.username, "test_user");
        assert_eq!(json_response.token, "test_token");
        assert_eq!(json_response.id, 1);
    }
}