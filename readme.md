Alla-Bot
========

Scrapes the Allakhazam item database to allow searches from Discord.

Compiling
=========

Written against Rust v1.39.0, but should compile against 1.37.0

Requires the following packages on Debian-based distributions:
* build-essential
* libssl-dev
* pkg-config

Running
=======

Set the environment variable DISCORD_TOKEN to the bot token from discord and invoke the `main` 
executable produced by compilation.

Usage
=====

After adding bot to a Discord server, invoke it with `!alla <search string>`. If there are 1-3
results the full stats for those items will be returned. Any more will require that the search 
be refined.
