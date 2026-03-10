use chromiumoxide::{Browser, BrowserConfig, Page};
use futures::StreamExt;
use tokio::task::JoinHandle;

pub async fn create_browser() -> Result<(Browser, Page, JoinHandle<()>), Box<dyn std::error::Error>>
{
    let (browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            .window_size(1220, 900)
            .with_head()
            .build()?,
    )
    .await?;

    let handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page = browser.new_page("about:blank").await?;

    Ok((browser, page, handle))
}
