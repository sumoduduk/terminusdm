use std::env::args;

#[tokio::main]
async fn main() {
    let mut args = args();
    args.next();
    let download_uri = args.next().expect("ERROR: argument is empty");
    dbg!(&download_uri);

    let _ = tdm::download_chunk(&download_uri).await;
}
