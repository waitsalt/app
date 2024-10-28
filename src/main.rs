mod app;
mod lib;

#[tokio::main]
async fn main() {
    app::run().await
}
