# `bobbot`

A Discord bot for creating and destroying **temporary voice channels**.

> _Ever wanted [Mumble](https://www.mumble.info/)'s temporary voice channels on Discord?_  
> _Bob can help you!_

![](resources/logo.svg)

## About

Bob is a bot that allows a group of users to create voice channels that are automatically deleted after everybody leaves.

## Usage

While you are connected to any voice channel:

- `/build <channel-name>` | **create a new temporary channel** and enter it
- `/save <preset-name>` | **save the permissions** of the channel you currently are in to a name
- `/load <preset-name> <channel-name>` | **load the permissions** with the name

## Add to your server

> The bot is still in the alpha stage of development, there are no guarantees of uptime or stability!
> 
> Please message `Steffo#4036` if you decide to add this bot to a new server :)

To add the bot to your server:

1. [Click on this link](https://discord.com/api/oauth2/authorize?client_id=737293731459498025&permissions=16778256&scope=bot).
2. Create a new category for the bot from which the temporary channels will inherit all permissions.
3. Create a text channel named `#bob` inside the category and allow the bot to _Send messages_ in it.

## Host your own

> The bot is still in the alpha stage of development, there are no guarantees of stability!

To host your own instance of Bob:

1. [Download the latest release on GitHub](https://github.com/Steffo99/bob/releases).
2. Set the `DISCORD_TOKEN` environment variable to your [Discord bot token](https://discord.com/developers/applications).
3. Set the `BOB_CHANNEL_NAME` environment variable to the name of Bob's text channel (without the starting hash, ex: `bob`).
4. Set the `BOB_DELETION_TIME` environment variable to how long would you like empty channels to stay available before being deleted. Defaults to `60` if not set.
4. Set the `BOB_PREFIX` environment variable to the command prefix you'd like the bot to use. Defaults to `!` if not set.
5. _If you're on OS X or Linux, set the executable flag on the file you downloaded!_
6. Run the executable.

## Compile from source

> The GitHub source is even more unstable than the releases; the compilation may even fail!

To compile Bob from the source on GitHub:

1. Ensure you have Cargo, Rust and Git installed on your PC.
2. Clone Bob's git repository with `git clone https://github.com/Steffo99/bob.git`.
2. Run `cargo build`.
