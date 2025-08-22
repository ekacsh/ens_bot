# ENS Bot

ENS Bot is a Discord bot written in Rust to help manage the Arcane Falcon guild in the game IdleOn. It provides automated features for guild management.

## Features

- Manage guild membership and roles
- Custom commands for IdleOn guild management

## Installation

```bash
git clone https://github.com/yourusername/ens_bot.git
cd ens_bot
cargo build --release
```

## Usage

```bash
./target/release/ens_bot --help
```

## Configuration

Before running the bot, create a `.env` file in the project root with the following variables:

```env
DISCORD_TOKEN=your_discord_bot_token
GUILD_ID=your_guild_id
GS_API_URL=your_google_sheet_api_url
MEE6_TOKEN=your_mee6_token
MEE6_API_URL=your_mee6_api_url
```

- `DISCORD_TOKEN`: Discord bot authentication token.
- `GUILD_ID`: Discord server (guild) ID where the bot will operate.
- `GS_API_URL`: Endpoint for the Google Sheets API integration.
- `MEE6_TOKEN`: Authentication token for Mee6 API access.
- `MEE6_API_URL`: Mee6 leaderboard API endpoint.

Ensure all required variables are set before starting the bot.

### Environment Variables
DISCORD_TOKEN: Token for the discord bot
GUILD_ID: Discord ID of the server to be deployed

GS_API_URL: URL for the Google Sheet Api

MEE6_TOKEN: Mee6 authentication token
MEE6_API_URL: Mee6 leaderboard api url

## Contributing

Contributions are welcome! Please open issues or submit pull requests.

## License

This project is licensed under the MIT License.