#[cfg(test)]
mod tests {
    use axum::{routing::post, Extension, Router};
    use axum_test::TestServer;
    use backend::entities::users::{self};
    use backend::routes::users::*;
    use sea_orm::{DatabaseBackend, MockDatabase};
    use serde_json::{json, Value};
    use std::sync::Arc;

    // Test the create_user handler
    #[tokio::test]
    async fn test_create_user() {
        // Setup mock database
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // Mock the expected query result for creating a new user
            .append_query_results([[users::Model {
                id: 1,
                username: "test_user".to_string(),
                password: "hashed_password".to_string(),
                token: Some("test_token".to_string()),
            }]])
            .into_connection();

        let db = Arc::new(db);

        // Create router with database extension
        let app = Router::new()
            .route("/users", post(create_user))
            .layer(Extension(db));

        // Create test server
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

    // Test the login_user handler
    #[tokio::test]
    async fn test_login_user() {
        // Setup mock database
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // Mock the expected query result for querying the database for the username
            .append_query_results([[users::Model {
                id: 1,
                username: "test_user".to_string(),
                // Hash the password in the request for in-handler verification
                password: bcrypt::hash("test_password", 10).unwrap(),
                token: Some("old_token".to_string()),
            }]])
            // Mock the expected query result for updating the user's token
            .append_query_results([[users::Model {
                id: 1,
                username: "test_user".to_string(),
                password: "hashed_password".to_string(),
                token: Some("new_token".to_string()),
            }]])
            .into_connection();

        let db = Arc::new(db);

        // Create router with database extension
        let app = Router::new()
            .route("/login", post(login_user))
            .layer(Extension(db));

        // Create test server
        let server = TestServer::new(app).unwrap();

        // Create test request
        let user_request: Value = json!({
            "username": "test_user",
            "password": "test_password"
        });

        // Send request to the server
        let response = server.post("/login").json(&user_request).await;

        // Validate the response
        response.assert_status_ok();
        let json_response = response.json::<UserResponse>();
        assert_eq!(json_response.username, "test_user");
        assert_eq!(json_response.token, "new_token");
        assert_eq!(json_response.id, 1);
    }
}
