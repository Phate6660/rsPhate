# rsPhate

A Discord bot written in Rust.

How to use:

- `export DISCORD_TOKEN=token` (obtain your token from the bot section of the developers/application part of discordapp's website)
- `cargo run` or `cargo run --release` from the root dir of the repo, or at the very least, the .env file must be in the CWD

## Current Commands

- `^about`: Bot will reply with pretty embed containing title and description of bot, as well as where to find the author (me).
- `^date`: Bot will reply with current date and time in this format -- `06:30 AM | Thu 21, May of 2020`.
- `^ls`: Bot will reply with a pretty embed that lists and describes available commands.
- `^msg`: Direct message user with list of commands.
- `^rr`: Bot will reply with a link (without a link preview) to Rick Astley's "Never Gonna Give You Up".
- `^wipltrn`: Bot will reply with pretty embed containing music info (via `mpc`) and cover art (via `/tmp/cover.png`).
- `^ww {steam,systemd}`: Bot will reply with pretty embed explaining why the topic is bad.
- `^quit`: Bot will reply with "Shutting down now!" and shut itself down directly after.

## Bot structure

- `.env`: for environmental variables
- `src/main.rs`: the main file for the bot, it's responsible for everything except for defining the commands
- `src/commands/*` (except mod.rs): define the commands to be used
- `src/commands/mod.rs`: for making the command files in `src/commands/` public for use in `src/main.rs`
