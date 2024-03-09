use std::env::args;

#[tokio::main]
async fn main() {
    let mut args = args();
    let download_uri = args.next().expect("argument is empty");
    let _ = tdm::download_chunk(&download_uri).await;
}
