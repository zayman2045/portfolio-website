#[cfg(test)]
mod tests {
    use axum::{routing::post, Router, Extension};
    use axum_test::TestServer;
    use backend::routes::users::*;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
    use serde_json::{json, Value};
    use backend::entities::users::{self};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_user() {
        // Setup mock database
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([[users::Model {
                id: 1,
                username: "test_user".to_owned(),
                password: "hashed_password".to_owned(),
                token: Some("test_token".to_owned()),
            }]])
            .append_exec_results([MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let db = Arc::new(db);

        // Create router with database extension
        let app = Router::new()
            .route("/users", post(create_user))
            .layer(Extension(db));

        let server = TestServer::new(app).unwrap();

        // Create test request
        let user_request: Value = json!({
            "username": "test_user",
            "password": "test_password"
        });

        // Send request and verify response
        let response = server.post("/users").json(&user_request).await;
        response.assert_status_ok();
        
        let json_response = response.json::<UserResponse>();
        assert_eq!(json_response.username, "test_user");
        assert_eq!(json_response.id, 1);
    }
}