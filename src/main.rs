use axum::{Json, Router, routing::get};
use rusqlite::Connection;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("server running or whatever");
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .fallback_service(ServeFile::new("index.html"))
        .route("/random_prompt", get(random_prompt));

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Serialize)]
struct Prompt {
    target: String,
    prompt: String,
}

#[derive(Debug)]
struct DbPrompt {
    id: i64,
    prompt: String,
}

#[derive(Debug)]
struct DbTarget {
    name: String,
    aspectRatio: String,
}

async fn random_prompt() -> Json<Vec<Prompt>> {
    println!("Lets frottage!");

    let conn = Connection::open("database.db").unwrap();

    let mut stmt_targets = conn
        .prepare("select name, aspectRatio from targets")
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

    let mut stmt_random_prompt = conn.prepare("select prompts.id, prompts.prompt from promptTargets, prompts where prompts.id=promptTargets.promptId and promptTargets.targetName=?1 order by random() limit 1").unwrap();

    let prompts: Vec<Prompt> = targets
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
            Prompt {
                target: target.name.clone(),
                prompt: db_prompt.prompt,
            }
        })
        .collect();
    Json(prompts)
}
