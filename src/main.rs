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

    let result = tokio::task::spawn_blocking(move || {
        let config_output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("--add")
            .arg("safe.directory")
            .arg(&workspace)
            .output();

        let pull_output = Command::new("git")
            .arg("-C")
            .arg(&workspace)
            .arg("pull")
            .output();

        (config_output, pull_output)
    })
    .await
    .unwrap();

    let (config_result, pull_result) = result;

    let config_output = match config_result {
        Ok(out) => format!(
            "Git Config Output:\n{}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
        Err(err) => format!("Error running git config: {}\n", err),
    };

    let pull_output = match pull_result {
        Ok(out) => format!(
            "Git Pull Output:\n{}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ),
        Err(err) => format!("Error running git pull: {}\n", err),
    };

    HttpResponse::Ok().body(format!(
        "<pre>{}</pre>\n<pre>{}</pre>\n<a href='/'>Back</a>",
        config_output, pull_output
    ))
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
