//! Tests for the backend routes
#[cfg(test)]
mod tests {
    use axum::{
        routing::{delete, get, post},
        Extension, Router,
    };
    use axum_test::TestServer;
    use backend::entities::{
        missions,
        users::{self},
    };
    use backend::routes::{missions::*, users::*};
    use sea_orm::{DatabaseBackend, DatabaseConnection, MockDatabase};
    use serde_json::{json, Value};
    use std::sync::Arc;

    /// Create a test server from a custom router and a mock database connection.
    pub fn create_test_server(database: DatabaseConnection) -> TestServer {
        // Wrap the database connection in an Arc to share it between threads
        let database = Arc::new(database);

        // Create the router by defining routes, assigning handlers, and attaching layers.
        let router = Router::new()
            .route("/users/logout", post(logout_user))
            .route("/missions", post(create_mission))
            .route("/users/:user_id", get(list_missions))
            .route("/missions/:mission_id", get(get_mission))
            .route("/missions/:mission_id", post(update_mission))
            .route("/missions/:mission_id", delete(delete_mission))
            .route("/users", post(create_user))
            .route("/login", post(login_user))
            .layer(Extension(database))
            .layer(Extension(users::Model {
                id: 1,
                username: "test_user".to_string(),
                password: "hashed_password".to_string(),
                token: Some("test_token".to_string()),
            }));

        // Create and return the test server
        TestServer::new(router).expect("Failed to create test server")
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

        // Create test server
        let server = create_test_server(db);

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

        // Create test server
        let server = create_test_server(db);

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

        // Create test server
        let server = create_test_server(db);

        // Send request to the server
        let response = server.post("/users/logout").await;

        // Validate the response
        response.assert_status_ok();
    }

    /// Test the create_mission handler.
    #[tokio::test]
    pub async fn test_create_mission() {
        // Setup mock database
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // Mock the expected query result for inserting a new mission
            .append_query_results([[missions::Model {
                id: 1,
                user_id: 1,
                title: "test_title".to_string(),
                content: Some("test_content".to_string()),
            }]])
            .into_connection();

        // Create test server
        let server = create_test_server(db);

        // Create test request
        let mission_request: Value = json!({
            "user_id": 1,
            "title": "test_title",
            "content": "test_content"
        });

        // Send request to the server
        let response = server.post("/missions").json(&mission_request).await;

        // Validate the response
        response.assert_status_ok();
        let json_response = response.json::<MissionBuildResponse>();
        assert_eq!(json_response.id, 1);
        assert_eq!(json_response.user_id, 1);
        assert_eq!(json_response.title, "test_title");
        assert_eq!(json_response.content, "test_content");
    }

    /// Test the list_missions handler.
    #[tokio::test]
    pub async fn test_list_missions() {
        // Setup mock database
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            // Mock the expected query result for querying the database for the user's missions
            .append_query_results([[
                missions::Model {
                    id: 1,
                    user_id: 1,
                    title: "test_title_1".to_string(),
                    content: Some("test_content_1".to_string()),
                },
                missions::Model {
                    id: 2,
                    user_id: 1,
                    title: "test_title_2".to_string(),
                    content: Some("test_content_2".to_string()),
                },
            ]])
            .into_connection();

        // Create test server
        let server = create_test_server(db);

        // Send request to the server
        let response = server.get("/users/1").await;

        // Validate the response
        response.assert_status_ok();
        let json_response = response.json::<MissionListResponse>();
        assert_eq!(json_response.missions.len(), 2);
        assert_eq!(json_response.missions[0].id, 1);
        assert_eq!(json_response.missions[0].user_id, 1);
        assert_eq!(json_response.missions[0].title, "test_title_1");
        assert_eq!(json_response.missions[0].content, "test_content_1");
        assert_eq!(json_response.missions[1].id, 2);
        assert_eq!(json_response.missions[1].user_id, 1);
        assert_eq!(json_response.missions[1].title, "test_title_2");
        assert_eq!(json_response.missions[1].content, "test_content_2");
    }
}
