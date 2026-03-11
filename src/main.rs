use chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams;
use colored::Colorize;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::io::BufRead;

mod browser;
mod config;
mod linkedin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = config::Config::load();
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

    println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());
    println!(
        "  {} {}",
        "🔗 Linky".bold().white(),
        "- LinkedIn Automation".dimmed()
    );
    println!("{}\n", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());

    loop {
        let options = vec!["Connect", "Comment", "Settings", "Quit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do?")
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                let amount: i8 = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("How many connections?")
                    .default(cfg.default_connect_amount)
                    .interact()?;
                println!(
                    "\n{} Connecting to {} people...\n",
                    "→".blue(),
                    amount.to_string().bold()
                );
                linkedin::connections::connect(&page, amount).await?;
                println!("{} Done!\n", "✓".green().bold());
            }
            1 => {
                let amount: i8 = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("How many comments?")
                    .default(cfg.default_comment_amount)
                    .interact()?;
                println!(
                    "\n{} Commenting on {} posts...\n",
                    "→".blue(),
                    amount.to_string().bold()
                );
                linkedin::feed::comment_posts(
                    &page,
                    amount,
                    cfg.rating_threshold,
                    cfg.gemini_api_key.clone(),
                    cfg.sample.clone(),
                )
                .await?;
                println!("{} Done!\n", "✓".green().bold());
            }
            2 => {
                let settings_options = vec!["Set API Key", "Set Writing Sample", "Back"];

                let settings_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Settings")
                    .items(&settings_options)
                    .default(0)
                    .interact()?;

                match settings_selection {
                    0 => {
                        let key: String = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter Gemini API key")
                            .interact()?;
                        cfg.gemini_api_key = key;
                        cfg.save();
                        println!("{} API key saved\n", "✓".green().bold());
                    }
                    1 => {
                        println!(
                            "Paste a description of your writing style, then press Enter twice when done:"
                        );
                        let mut sample = String::new();
                        let stdin = std::io::stdin();
                        for line in stdin.lock().lines() {
                            let line = line?;
                            if line.is_empty() {
                                break;
                            }
                            sample.push_str(&line);
                            sample.push('\n');
                        }
                        cfg.sample = sample;
                        cfg.save();
                        println!("{} Writing sample saved\n", "✓".green().bold());
                    }
                    _ => {}
                }
            }
            3 => break,
            _ => {}
        }
    }

    b.close().await?;
    handle.await?;
    Ok(())
}
