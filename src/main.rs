use std::{thread::sleep, time::Duration};

use chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams;

mod browser;
mod linkedin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut b, page, handle) = browser::create_browser().await?;

    page.goto("https://linkedin.com").await?;

    // Height 140px less then window size
    page.execute(
        SetDeviceMetricsOverrideParams::builder()
            .width(1220u32)
            .height(760u32)
            .device_scale_factor(1.0)
            .mobile(false)
            .build()?,
    )
    .await?;

    linkedin::connections::connect(page, 5).await?;

    sleep(Duration::from_secs(5));

    b.close().await?;
    handle.await?;
    Ok(())
}
