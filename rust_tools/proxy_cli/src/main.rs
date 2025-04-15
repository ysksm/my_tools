use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use awc::Client;
use std::net::SocketAddr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Edgeに設定するプロキシアドレス：127.0.0.1:8080
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    HttpServer::new(|| {
        App::new().default_service(web::to(proxy_handler))
    })
    .bind(addr)?
    .run()
    .await
}

async fn proxy_handler(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let client = Client::new();

    // Edgeから来たリクエストには、絶対URLが含まれている（例: http://example.com/path）
    let uri = req.uri().to_string();

    // デバック用にリクエスト情報を表示
    println!("Request: {} {}", req.method(), uri);

    // actix_web では自動で `host` に変換されることがあるので、絶対URLを復元する
    let target_url = if uri.starts_with("http://") || uri.starts_with("https://") {
        uri
    } else {
        // fallback
        format!("http://{}", req.uri())
    };

    let mut forwarded_req = client
        .request(req.method().clone(), target_url.clone())
        .no_decompress();

    // ヘッダーをコピー（User-Agentなど）
    for (name, value) in req.headers() {
        forwarded_req = forwarded_req.insert_header((name.clone(), value.clone()));
    }

    // 転送してレスポンス取得
    let res = forwarded_req.send_body(body).await;

    match res {
        Ok(mut upstream_res) => {
            let mut client_resp = HttpResponse::build(upstream_res.status());

            for (name, value) in upstream_res.headers() {
                client_resp.append_header((name.clone(), value.clone()));
            }

            match upstream_res.body().await {
                Ok(b) => client_resp.body(b),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::BadGateway().body("Proxy failed to fetch target"),
    }
}