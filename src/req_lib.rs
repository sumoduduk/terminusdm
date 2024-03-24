use eyre::{eyre, OptionExt};
use reqwest::{
    header::{HeaderMap, ACCEPT_RANGES, CONTENT_LENGTH},
    Client, Url,
};

pub struct HeaderObject {
    header: HeaderMap,
    url: Url,
}

impl HeaderObject {
    pub async fn new(uri: &str) -> eyre::Result<Self> {
        let resp = Client::new().get(uri).send().await?;
        Ok(HeaderObject {
            header: resp.headers().clone(),
            url: Url::parse(uri)?,
        })
    }

    pub fn get_sizes(&self) -> eyre::Result<u64> {
        let sizes = self.header.get(CONTENT_LENGTH).ok_or_eyre("Size Unknown")?;
        let size_num = sizes.to_str()?.parse::<u64>()?;
        Ok(size_num)
    }

    pub fn is_ranges(&self) -> eyre::Result<bool> {
        let accpe_ranges = match self.header.get(ACCEPT_RANGES) {
            None => Err(eyre!("Not Resumable")),
            Some(x) if x == "none" => Err(eyre!("Not Resumable")),
            Some(_) => Ok(true),
        };
        accpe_ranges
    }

    pub fn get_filename(&self) -> Option<String> {
        let path_segment = self.url.path_segments()?;

        let filename = path_segment.last().map(String::from).map(|f| {
            let file_name = form_urlencoded::parse(f.as_bytes())
                .map(|(key, val)| [key, val].concat())
                .collect();
            file_name
        });

        filename
    }

    pub fn get_url(&self) -> &Url {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const URI : &str = "https://huggingface.co/datasets/ym0v0my/Time_series_dataset/resolve/main/all_six_datasets.zip?download=true";

    #[tokio::test]
    async fn test_get_filename() -> eyre::Result<()> {
        let head = HeaderObject::new(URI).await?;
        let name = head.get_filename();
        let name_str = name.unwrap();

        dbg!(&name_str);

        assert_eq!("all_six_datasets.zip", &name_str);

        Ok(())
    }
}
