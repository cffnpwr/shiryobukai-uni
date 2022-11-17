use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_latest));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_latest() -> Html<String> {
    let url = "https://gist.githubusercontent.com/souhait0614/eaa165f0178985be17aebec6fe55710c/raw/f151c7b9bd3eeb9f240f25fb634ab8937416767c/shiryobukai-uni.txt";
    match reqwest::get(url).await {
        Ok(res) => match res.text().await {
            Ok(mut body) => {
                body = body.replace('\n', "<br>");

                Html(format!("{}{}{}", "<p><a href=\"https://gist.github.com/souhait0614/eaa165f0178985be17aebec6fe55710c\" target=\"_blank\">思慮深いウニ</a></p><p>", body, "</p>"))
            }
            Err(msg) => Html(msg.to_string()),
        },
        Err(msg) => Html(msg.to_string()),
    }
}
