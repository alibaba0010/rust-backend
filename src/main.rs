use serde::Serialize;
use warp::{Filter, Reply};

#[derive(Serialize)]
struct User {
    name: String,
    email: String,
    profession: String,
}

#[derive(Serialize)]
struct UserWithId {
    id: u32,
    name: String,
    email: String,
    profession: String,
}

#[tokio::main]
async fn main() {
    // Route for "/" - Hello World
    let hello = warp::path::end()
        .map(|| warp::reply::html("Hello World"));

    // Route for "/api/user" - Static JSON
    let api_user = warp::path("api")
        .and(warp::path("user"))
        .and(warp::path::end())
        .map(|| {
            let user = User {
                name: "John Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                profession: "Software Engineer".to_string(),
            };
            warp::reply::json(&user)
        });

    // Route for "/api/user/:id" - JSON with ID parameter
    let api_user_with_id = warp::path("api")
        .and(warp::path("user"))
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .map(|id: u32| {
            let user = UserWithId {
                id,
                name: "John Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                profession: "Software Engineer".to_string(),
            };
            warp::reply::json(&user)
        });

    // Combine all routes
    let routes = hello
        .or(api_user)
        .or(api_user_with_id);

    // Start server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}