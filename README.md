# rsPhate

![bot](images/bot.png?raw=true "bot")

A Discord bot written in Rust.

Anything with a "*" denotes an entry in the notes section.

How to use:

- `export DISCORD_TOKEN=token` (obtain your token from the bot section of the developers part of discord's website).
- `cargo run` or `cargo run --release` from the root dir of the repo, or at least, the `.env` and `scripts/` must be in the CWD.

## Current Commands

- `^about`: Bot will reply with pretty embed containing title and description of bot, as well as where to find the author (me).
- `^date`: Bot will reply with current date and time in this format -- `06:30 AM | Thu 21, May of 2020`.
- `^help {_,command,function}`: Without args, bot will give a generalized help. Specifying a command or function will explain it.
- `^fortune`: Bot will reply with random fortune from `fortune-mod-mythical-linux` (repo is pinned to my profile if anyone is interested).
- `^hmm`: How much music does Phate have?
- `^iv SEARCH`: Bot will reply with an invidio link of the search query.*
- `^math operation num num`: Bot will do math for you (basic add/sub/div/mul) and reply with the result.*
- `^projects`: Bot willk reply with pretty embed containing links to other projects by the author.
- `^rng min max`: Bot will generate a random number between min and max and reply with the result.
- `^rr`: Bot will reply with a link (without a link preview) to Rick Astley's "Never Gonna Give You Up".
- `^wipltrn`: Bot will reply with pretty embed containing music info and cover art.*
- `^ww {apple,steam,systemd}`: Bot will reply with pretty embed explaining why the topic is bad.
- `^quit`: Bot will reply with "Shutting down now!" and shut itself down directly after.

## Bot structure

- `.env`: For environmental variables.
- `scripts/`: Contains scripts to be ran for certain commands.
- `src/main.rs`: The main file for the bot, it's responsible for everything except for defining the commands.
- `src/commands/*` (except `mod.rs`): Define the commands to be used.
- `src/commands/mod.rs`: For making the command files in `src/commands/` public for use in `src/main.rs`.

## Notes

- `math`: Operation refers to `multiply`/`divide`/`add`/`subtract`. Example: `^math add 1 1` will make the bot reply with `2`.
- `iv`: Requires `youtube-dl`. Example: `^iv type o negative dead again` will cause the bot to reply with an invidio link to the full album.
- `wipltrn`: Requires `mpc`, and for the cover art to be present at `/tmp/cover.png`.

There are 3 valid delimiters for arguments. ` `, `, `, and `,`.<br>
That means any of these are valid:<br>
`^math add 1 1`, `^math add,1,1`, `^math add 1,1`, `^math add, 1 1`, etc.
