use eyre::OptionExt;
use std::env::args;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut args = args();

    args.next();

    let download_uri = args;
    let download_dirs = dirs::download_dir().ok_or_eyre("ERROR: no download_dir")?;

   let download_im = vec![] 

    Ok(())
}
