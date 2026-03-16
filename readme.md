# Linky

> **Disclaimer:** I take no responsibility for how you use this software. Automating LinkedIn may violate their Terms of Service and could result in your account being restricted or banned. Use at your own risk.

A LinkedIn automation tool built in Rust. Linky uses AI to intelligently interact with your feed and automatically grow your network.

## Features

- **Connect** — Automatically send connection requests to people in your feed
- **Comment** — Uses AI to rate posts by commentability and write genuine comments
- **Arrow-key menu** — Simple terminal UI, no commands to memorize

## Requirements

- [Google Chrome](https://www.google.com/chrome/) installed on your machine
- A Gemini API key (free tier works fine)
- macOS (only platform supported currently)

## Getting Started

### Option 1 — Download the binary (easiest)

1. Go to the [Releases](https://github.com/YasushikoX/linky/releases) page
2. Download the binary
3. Open Terminal, navigate to where you downloaded it and run:
```bash
chmod +x linky
./linky
```

> **macOS note:** If you see a warning saying the app can't be opened because it's from an unidentified developer, go to **System Settings → Privacy & Security** and click **Open Anyway**.

### Option 2 — Build from source

Make sure you have [Rust](https://rustup.rs/) installed, then:

```bash
git clone https://github.com/YasushikoX/linky
cd linky
cargo run
```

### Get a Gemini API Key

Linky uses Google Gemini AI to rate and write comments.

1. Go to [aistudio.google.com](https://aistudio.google.com)
2. Sign in with your Google account
3. Click **Get API Key** → **Create API key**
4. Copy the key
5. Launch Linky, go to **Settings → Set API Key** and paste it in

Your key is saved locally and never leaves your machine.

## Usage

Launch Linky and use the arrow keys to navigate the menu:

```
? What do you want to do?
> Connect
  Comment
  Settings
  Quit
```

> **First launch:** When Linky starts, a Chrome window will open on the LinkedIn login page. Log in manually using your LinkedIn email and password — Google sign-in won't work due to Google's remote control protection. Once logged in, come back to the terminal and use the menu normally.

- **Connect** — enter how many connection requests to send
- **Comment** — enter how many posts to comment on
- **Settings** — set your API key, default amounts, and timing preferences
