#[cfg(test)]
mod tests {
    use axum::routing::post;
    use axum::Router;
    use axum_test::TestServer;
    use backend::routes::users::*;
    use serde_json::{json, Value};

    #[tokio::test]
    async fn test_create_user() {
        // Create the router and server
        let app = Router::new().route("/users", post(create_user));
        let server = TestServer::new(app).unwrap();

        // Create a user request
        let user_request: Value = json!({
            "username": "test_user",
            "password": "test_password"
        });

        let response = server.post("/users").json(&user_request).await;
        response.assert_status_ok();
        let json_response = response.json::<UserResponse>();
        assert_eq!(json_response.username, "test_user");
    }
}
