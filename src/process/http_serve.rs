use anyhow::{Ok, Result};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::{path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};
#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}
pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serving {:?} on http:://{}", path, addr);
    // 注册资源目录 cargo run http serve --dir fixtures 将./fixtures注册为资源目录，为file_handler提供
    let state = HttpServeState { path: path.clone() };
    let serve_dir = ServeDir::new(path).append_index_html_on_directories(true);
    let router = Router::new()
        // 为/tower作为前缀的uri配置一个服务目录，cargo run http serve --dir fixtures/aaa这个fixures/aaa注册到/tower下
        // 并且axum处理静态文件的发送
        .nest_service("/tower", serve_dir)
        // 为通配符路径/*path配置一个get请求的处理函数，根目录在第一个注释中已注册
        // 对于/tower这个uri它没有被 ServeDir 明确处理或者匹配到特定的目录处理规则，就可能会落入通配符路径的范畴，从而触发 file_handler 函数的调用
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, Html<String>) {
    let is_dir = path.ends_with("/");
    let p = std::path::Path::new(&state.path).join(path.clone());
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            Html(format!("File {} not found", path)),
        )
    } else {
        if p.is_dir() {
            let files = read_file_names_in_directory(&p);
            let mut content = String::new();
            let mut url = String::new();
            if is_dir {
                let mut path = path;
                path.pop();
                let p = std::path::Path::new(&state.path).join(path);
                url.push_str(&format!(
                    "http://localhost:8080/tower/{}",
                    &p.to_str().unwrap()[2..]
                ));
            } else {
                url.push_str(&format!(
                    "http://localhost:8080/{}",
                    &p.to_str().unwrap()[2..]
                ));
            }
            for file in files {
                let tag = format!("<li><a href=\"{}/{}\">{}</a></li>\n", url, file, file);
                content.push_str(&tag);
            }
            let content = build_html(content);
            return (StatusCode::OK, Html(content));
        }
        match tokio::fs::read_to_string(p).await {
            core::result::Result::Ok(content) => {
                info!("Read length: {}", content.len());
                (StatusCode::OK, Html(content))
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Html(e.to_string()))
            }
        }
    }
}
fn read_file_names_in_directory(directory_path: &PathBuf) -> Vec<String> {
    std::fs::read_dir(directory_path)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect()
}
fn build_html(content: String) -> String {
    let html1 = "<!DOCTYPE html>
<html>
    <head>
        <title>Static File Access</title>
    </head>
    <body>
        <p>This is a static file access HTML page.</p>
        "
    .to_string();
    let html2 = "    </body>
</html>";
    html1 + &content + html2
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        println!("{:?}", content);
    }
    #[test]
    fn test_dir() {
        let path = "fixtures";
        let p = PathBuf::new().join(path);
        let files = read_file_names_in_directory(&p);
        for file in files {
            println!("{}", file);
        }
    }
}
