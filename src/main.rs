use axum::{
    Json, Router,
    routing::{get, post},
};
use rand::Rng;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use tera::{Context, Tera, Value};
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

mod template;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("server running or whatever");
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .fallback_service(ServeFile::new("index.html"))
        .route("/random_prompt", get(random_prompt))
        .route("/vote", post(vote_prompt))
        .route("/image", post(save_image));

    // write address like this to not make typos
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("listening on port {}", addr.port());
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ApiImageRequest {
    promptId: i64,
    targetName: String,
    imageUrl: String,
}

async fn save_image(Json(image): Json<ApiImageRequest>) -> Result<(), String> {
    println!(
        "saving image for prompt {} on {} (url: {})",
        image.promptId, image.targetName, image.imageUrl
    );

    let conn = Connection::open("database.db").map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO image (prompt_id, target_name, image_url) VALUES (?1, ?2, ?3)",
        (&image.promptId, &image.targetName, &image.imageUrl),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct ApiPrompt {
    targetName: String,
    prompt: String,
    promptId: i64,
}

#[derive(Debug)]
struct DbPrompt {
    id: i64,
    prompt: String,
}

#[derive(Debug)]
#[allow(non_snake_case)]
struct DbTarget {
    name: String,
    aspectRatio: String,
}

async fn random_prompt() -> Json<Vec<ApiPrompt>> {
    println!("Lets frottage!");

    let conn = Connection::open("database.db").unwrap();

    let mut stmt_targets = conn
        .prepare("select target_name, aspect_ratio from target")
        .unwrap();

    let targets: Vec<DbTarget> = stmt_targets
        .query_map([], |row| {
            Ok(DbTarget {
                name: row.get(0).unwrap(),
                aspectRatio: row.get(1).unwrap(),
            })
        })
        .unwrap()
        .map(|res| res.unwrap())
        .collect();

    let mut stmt_random_prompt = conn.prepare("select prompt.prompt_id, prompt.prompt from prompt_target, prompt where prompt.prompt_id=prompt_target.prompt_id and prompt_target.target_name=?1 order by random() limit 1").unwrap();

    let prompts: Vec<ApiPrompt> = targets
        .iter()
        .map(|target| {
            let db_prompt: DbPrompt = stmt_random_prompt
                .query_row([&target.name], |row| {
                    Ok(DbPrompt {
                        id: row.get(0).unwrap(),
                        prompt: row.get(1).unwrap(),
                    })
                })
                .unwrap();
            ApiPrompt {
                targetName: target.name.clone(),
                prompt: format!(
                    "{} --aspect {} --profile najcud4 --version 7",
                    template::render_prompt(&db_prompt.prompt),
                    target.aspectRatio
                ),
                promptId: db_prompt.id,
            }
        })
        .collect();
    Json(prompts)
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct ApiVoteRequest {
    promptId: i64,
    targetName: String,
    stars: i32,
}

async fn vote_prompt(Json(vote): Json<ApiVoteRequest>) -> Result<(), String> {
    println!(
        "voting for prompt {} on {} ({} stars)",
        vote.promptId, vote.targetName, vote.stars
    );

    if vote.stars < 1 || vote.stars > 5 {
        return Err("Stars must be between 1 and 5".to_string());
    }

    let conn = Connection::open("database.db").map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO vote (prompt_id, target_name, stars) VALUES (?1, ?2, ?3)",
        (&vote.promptId, &vote.targetName, &vote.stars),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
