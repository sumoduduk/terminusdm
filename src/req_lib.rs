use eyre::{eyre, OptionExt};
use reqwest::{
    header::{HeaderMap, ACCEPT_RANGES, CONTENT_LENGTH},
    Client,
};

pub struct HeaderObject {
    header: HeaderMap,
}

impl HeaderObject {
    pub async fn new(uri: &str) -> eyre::Result<Self> {
        let resp = Client::new().get(uri).send().await?;
        Ok(HeaderObject {
            header: resp.headers().clone(),
        })
    }

    pub fn get_sizes(&self) -> eyre::Result<u32> {
        let sizes = self.header.get(CONTENT_LENGTH).ok_or_eyre("Size Unknown")?;
        let size_num = sizes.to_str()?.parse::<u32>()?;
        Ok(size_num)
    }

    pub fn is_ranges(&self) -> eyre::Result<bool> {
        let accpe_ranges = match self.header.get(ACCEPT_RANGES) {
            None => Err(eyre!("Not Resumable")),
            Some(_) => Ok(true),
            Some(x) if x == "none" => Err(eyre!("Not Resumable")),
        };
        accpe_ranges
    }
}
