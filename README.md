# Bob Bot

A Discord bot for creating and destroying **temporary voice channels**.

> _Ever wanted [Mumble](https://www.mumble.info/)'s temporary voice channels on Discord?_  
> _Bob can help you!_

## Usage

The bot allows everyone on the server to create temporary voice channels in **any category containing a text channel where slash commands are enabled**.

### Creating a temporary voice channel

To create a temporary voice channel, use the `/build` command while you're connected to voice chat:
```text
/build {name}
```

A new temporary voice channel will be created, and you will be moved inside it by the bot.

> You will have **all** permissions on that voice channel, as if the administrator of the server gave you the __Manage Channel__ permission on it.
>
> You can use that privilege to create a private voice channel for just you and a few friends, or to allow only a select number of people to talk.

### Saving and loading presets

If you find yourself setting often the same permissions on a voice channel, you may want to store them in a preset so you will be able to load them in the 
future.

You can do so with the `/save` slash command:
```text
/save {preset} {channel} [overwrite]
```

> If you have the __Manage Channels__ permission on the whole server, you can edit existing presets by specifying the `{overwrite}` argument.

You will then be able to load the preset with the `/build` slash command:
```text
/build {name} {preset}
```

> Presets are server-specific, so you don't have to worry about name collisions.

### Configuring the bot

#### Setting the command channel

If you have the Manage Channels permission on the whole server, you'll be able to select the text channel where the bot will send notifications with the 
`/config cc` command:
```text 
/config cc {channel}
```

#### Setting the deletion time

If you have the Manage Guild permission on the whole server, you'll be able to choose the time that temporary channels will be able to stay empty for before 
they are deleted with the `/config dt` command:
```text 
/config dt {timeout}
```

> Timeouts above 30 minutes are experimental and may cause slowdowns in the bot.

## Add to your own server

You can add the bot to your own server by clicking on 
**[this link](https://discord.com/api/oauth2/authorize?client_id=737293731459498025&permissions=8&scope=bot%20applications.commands)**.

Please note that the bot **must be** an __Administrator__ to work correctly, due to a peculiarity in Discord's permission system, which doesn't allow non-Administrators to create channels with the __Manage permissions__ permission set.

Once you added the bot, set a command channel with `/config cc` and a deletion time with `/config dt`, or the bot will refuse to run.

## Hosting your own instance

### Prerequisites

- A [Discord Application](https://discord.com/developers/applications) with an associated bot account
- A computer to host the bot on
    - [**Rust ^1.52.1**](https://www.rust-lang.org/it/tools/install) with `cargo`
    - [**PostgreSQL ^13.3**](https://www.postgresql.org/)

### Installation

01. Download and compile the `bobbot` crate through `cargo`:
    ```console
    $ cargo install bobbot   
    ```

02. Create a Postgres role and database for Bob Bot:
    ```postgresql
    CREATE USER bobbot;
    CREATE DATABASE bobbot OWNER bobbot;
    ```

03. Set the following environment variables, or create a `.env` file in the directory where you will execute the bot from:
    ```dotenv
    # Sets the logging level
    # https://docs.rs/env_logger/0.9.0/env_logger/
    export RUST_LOG=bobbot=info
    # Discord bot account token
    # https://discord.com/developers/applications/APPLICATION_ID/bot
    export DISCORD_TOKEN=AAAAAAAAAAAAAAAAAAAAAAAA.AAAAAA.AAAAAAAAAAAAAAAAAAAAAAAAAAA
    # Discord application id
    # https://discord.com/developers/applications/APPLICATION_ID/information
    export DISCORD_APPID=000000000000000000
    # URL of the Postgres database
    # https://diesel.rs/guides/getting-started#setup-diesel-for-your-project
    export DATABASE_URL=postgres://bobbot@/bobbot
    ```
    
## Running

01. The first time you will run the bot, you'll need to **register its slash commands** so users can call them.  
    You can do so by running the bot **once** with the following environment variable set:
    ```console
    $ DISCORD_REGISTER_COMMANDS=1 bobbot
    ```

02. The next times you will run the bot, do so **without** the environment variable, or all commands will not work for up to an hour:
    ```console
    $ bobbot
    ```

## Updating

01. You can update the bot by re-installing the crate with `cargo`:
    ```console
    $ cargo install bobbot
    ```

02. If the slash commands have changed, you might need to **re-register the slash commands** at the first run:
    ```console
    $ DISCORD_REGISTER_COMMANDS=1 bobbot
    ```

## Development

The project was developed using [IntelliJ IDEA Ultimate](https://www.jetbrains.com/idea/) with the [IntelliJ Rust](https://www.jetbrains.com/rust/) plugin, and includes some useful things to make debug easier.