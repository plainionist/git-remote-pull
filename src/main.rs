use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::process::Command;
use std::sync::Mutex;

struct AppState {
    git_workspace: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "<html><body>
        <form action='/update' method='post'>
            <button type='submit'>Update</button>
        </form>
        </body></html>",
    )
}

async fn update(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let workspace = data.lock().unwrap().git_workspace.clone();

    let output = tokio::task::spawn_blocking(move || {
        Command::new("git")
            .arg("-C")
            .arg(&workspace)
            .arg("pull")
            .output()
    })
    .await
    .unwrap();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);

            HttpResponse::Ok().body(format!(
                "<pre>{}\n{}</pre><br><a href='/'>Back</a>",
                stdout, stderr
            ))
        }
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Error: {}<br><a href='/'>Back</a>", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let git_workspace = if args.len() > 1 {
        args[1].clone()
    } else {
        eprintln!("Usage: {} <git_workspace>", args[0]);
        std::process::exit(1);
    };

    let state = web::Data::new(Mutex::new(AppState { git_workspace }));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .route("/update", web::post().to(update))
    })
    .bind("0.0.0.0:9999")?
    .run()
    .await
}
