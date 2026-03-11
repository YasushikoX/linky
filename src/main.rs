use chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams;
use std::io::{self, BufRead, Write};

mod browser;
mod linkedin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut b, page, handle) = browser::create_browser().await?;

    page.goto("https://linkedin.com").await?;

    page.execute(
        SetDeviceMetricsOverrideParams::builder()
            .width(1220u32)
            .height(760u32)
            .device_scale_factor(1.0)
            .mobile(false)
            .build()?,
    )
    .await?;

    println!("Linky ready.");
    println!("Commands: connect <amount>, quit");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut parts = line.trim().splitn(2, ' ');
        let cmd = parts.next().unwrap_or("");
        let arg: i8 = parts.next().unwrap_or("10").parse().unwrap_or(10);

        match cmd {
            "connect" => linkedin::connections::connect(&page, arg).await?,
            "quit" => break,
            _ => println!("Unknown command: {}", cmd),
        }

        println!("Done. Waiting for next command...");
        print!("> ");
        io::stdout().flush()?;
    }

    b.close().await?;
    handle.await?;
    Ok(())
}
