use eyre::OptionExt;
use reqwest::{
    header::{HeaderMap, ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH},
    Client, Url,
};

pub struct HeaderObject {
    header: HeaderMap,
    url: Url,
}

impl HeaderObject {
    pub async fn new(uri: Url) -> eyre::Result<Self> {
        let resp = Client::new().get(uri.clone()).send().await?;
        dbg!(resp.headers());
        Ok(HeaderObject {
            header: resp.headers().clone(),
            url: uri,
        })
    }

    pub fn get_sizes(&self) -> eyre::Result<u64> {
        let sizes = self.header.get(CONTENT_LENGTH).ok_or_eyre("Size Unknown")?;
        let size_num = sizes.to_str()?.parse::<u64>()?;
        Ok(size_num)
    }

    pub fn is_ranges(&self) -> bool {
        let accept_range = match self.header.get(ACCEPT_RANGES) {
            Some(x) => x != "none",
            None => false,
        };
        accept_range
    }

    pub fn get_filename(&self) -> Option<String> {
        let name: Option<String> = self
            .header
            .get(CONTENT_DISPOSITION)
            .and_then(|disposition| {
                disposition
                    .to_str()
                    .ok()
                    .filter(|content| content.contains("filename"))
            })
            .map(Into::into);
        match name {
            Some(name) => {
                match name
                    .split(';')
                    .find(|part| part.trim_start().starts_with("filename="))
                    .and_then(|filename_part| filename_part.split('=').nth(1))
                    .and_then(|filename| Some(filename.trim_matches('"')))
                {
                    Some(s) => Some(s.to_owned()),
                    None => None,
                }
            }
            None => {
                let path_segment = self.url.path_segments()?;

                let filename = path_segment.last().map(String::from).map(|f| {
                    let file_name = form_urlencoded::parse(f.as_bytes())
                        .map(|(key, val)| [key, val].concat())
                        .collect();
                    file_name
                });

                filename
            }
        }
    }

    pub fn get_url(&self) -> &Url {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use crate::begin_download::trauma::download::Download;

    use super::*;

    const URI : &str = "https://huggingface.co/datasets/ym0v0my/Time_series_dataset/resolve/main/all_six_datasets.zip?download=true";
    const URI_2: &str = "https://github.com/sumoduduk/terminusdm/archive/refs/heads/main.zip";

    async fn get_download_url(url: &str) -> eyre::Result<Url> {
        let res = reqwest::get(url).await?;
        dbg!(res.url().as_str());
        Ok(res.url().to_owned())
    }

    #[tokio::test]
    async fn test_get_filename_1() -> eyre::Result<()> {
        let url = get_download_url(URI).await?;
        let head = HeaderObject::new(url.clone()).await?;
        let name = head.get_filename();

        let name_str = name.unwrap();

        dbg!(&name_str);

        assert_eq!("all_six_datasets.zip", &name_str);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_filename_2() -> eyre::Result<()> {
        let url = get_download_url(URI_2).await?;
        let head = HeaderObject::new(url.clone()).await?;
        let name = head.get_filename();

        let name_str = name.unwrap();

        dbg!(&name_str);

        assert_eq!("terminusdm-main.zip", &name_str);

        Ok(())
    }
}
