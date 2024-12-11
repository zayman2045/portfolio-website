//! Tests for the backend routes
#[cfg(test)]
mod tests {
    use axum::{
        routing::{delete, get, post},
        Extension, Router,
    };
    use axum_test::TestServer;
    use backend::entities::users::{self};
    use backend::routes::{missions::*, users::*};
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase};
    use serde_json::{json, Value};
    use std::sync::Arc;

    /// Create a test router with a mock database connection
    pub fn create_test_router(database: DatabaseConnection) -> Router {
        // Wrap the database connection in an Arc to share it between threads
        let database = Arc::new(database);

        // Define the routes, assign handlers, and attaches layers.
        Router::new()
            .route("/users/logout", post(logout_user))
            .route("/missions", post(create_mission))
            .route("/users/:user_id", get(list_missions))
            .route("/missions/:mission_id", get(get_mission))
            .route("/missions/:mission_id", post(update_mission))
            .route("/missions/:mission_id", delete(delete_mission))
            .route("/users", post(create_user))
            .route("/login", post(login_user))
            .layer(Extension(database))
    }

    /// Test the create_user handler.
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

        // Create test router
        let app = create_test_router(db);

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

    /// Test the login_user handler.
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

        // Create test router
        let app = create_test_router(db);

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

    /// Test the logout_user handler.
    #[tokio::test]
    async fn test_logout_user() {
        // Setup mock database
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // Mock the expected query result for querying the database for the user
            .append_query_results([[users::Model {
                id: 1,
                username: "test_user".to_string(),
                password: "hashed_password".to_string(),
                token: Some("test_token".to_string()),
            }]])
            // Mock the expected query result for updating the user's token to None
            .append_query_results([[users::Model {
                id: 1,
                username: "test_user".to_string(),
                password: "hashed_password".to_string(),
                token: None,
            }]])
            .into_connection();

        // Create router with database extension and user model extension
        let app = create_test_router(db).layer(Extension(users::Model {
            id: 1,
            username: "test_user".to_string(),
            password: "hashed_password".to_string(),
            token: Some("test_token".to_string()),
        }));

        // Create test server
        let server = TestServer::new(app).unwrap();

        // Send request to the server
        let response = server.post("/users/logout").await;

        // Validate the response
        response.assert_status_ok();
    }
}
