# JedBot                ![Owner](https://img.shields.io/badge/Owner-Glowstick-black)
[![Invite Jed!][invite-badge]][invite-link]
[![Dependency Status][dependency-badge]][dependency-link]
[![GitHub release](https://img.shields.io/github/release/JedBot/StrapDown.js.svg)](https://GitHub.com/JedBot/StrapDown.js/releases/)
[![Documentation Status](https://readthedocs.org/projects/ansicolortags/badge/?version=latest)](https://github.com/Glowstick0017/JedBot/wiki)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://lbesson.mit-license.org/)

A feature-packed bot for Discord servers that simulates Jed Crandall! Written in Rust with Serenity and various other libraries.



credits: [@PseudoSyntax](https://github.com/PseudoSyntax), [@gatoflaco](https://github.com/gatoflaco), [@DaumanaASU](https://github.com/DaumanaASU), [@SaajanM](https://github.com/SaajanM), [@Jbautista13](https://github.com/Jbautista13), [@JesusM2011](https://github.com/JesusM2011), [@brokenbyte](https://github.com/brokenbyte)


<p align="center">
  <a href="https://sketchywebsite.net/">
     <img width="120" alt="center" width="120" height="120" src="https://github.com/Glowstick0017/JedBot/blob/master/jed.png?raw=true">
  </a>





Welcome to the official GitHub repository for JedBot, a bot for the Discord chat platform written in Rust with the
serenity library, as well as various other libraries. It should be noted that this project is still in a Work-In-Progress
state(probably forever), however there are still a pretty robust set of commands implemented so far. This project will be 
continulously improved and updated with more commands and features(maybe),
so please keep an eye on this repository for any new features and updates(not really).

## Installation

### Prerequisites

Alright, before we can get JedBot up and running, we'll need to install a couple pieces of software in order for JedBot
to actually build and run. This will depend on your operating system, be it either Windows, macOS or Linux. On Windows,
this means you'll need Visual Studio 2019 installed, be it either the full IDE (Community, Professional, or Enterprise work
fine) or just the Visual Studio 2019 Build Tools, and Rust itself. On macOS, you will need the Xcode Developer Tools, as
it includes the system compiler (`clang`) necessary to build Rust programs and libraries, or you could also go with simply
installing Rust through the  <a href="https://brew.sh/">homebrew</a> tool. On Linux, you don't need to install anything in most cases, as most Linux
distributions such as Ubuntu and Fedora already have the `gcc` toolchain installed, however if desired this can be switched
to the same `clang` compiler as macOS by installing it through your respective package manager, or through  <a href="https://brew.sh/">homebrew</a> as
well.

### "Glowstick add whatever else you think is needed here" -Chicken

All in all, you will need the following prerequisites for JedBot to build and run:

* Visual Studio 2019 / Visual Studio 2019 Build Tools (*Windows (non-WSL) only*)
* Xcode Developer Tools(if on macOS)
* Rust, preferably version 0.9 or later

#### Windows

> **TODO**: Add instructions for repl.it

[![Open in Visual Studio Code](https://open.vscode.dev/badges/open-in-vscode.svg)](https://open.vscode.dev/Naereen/badges)
  
To install Visual Studio 2019, or the Visual Studio 2019 build tools, please visit the website for Visual Studio, which can
be accessed by [clicking here](https://visualstudio.microsoft.com/), hover over the Download Visual Studio button on the
tile for Visual Studio, and selecting any given edition. If you have a license for either Professional or Enterprise, select
either of those, but if you do not, the Community works fine too. Or, if you would just like to install the Build Tools instead
of installing the entire IDE, you can visit [this URL](https://visualstudio.microsoft.com/downloads/), scroll down to the
All Downloads section, expand the "Tools for Visual Studio 2019" section, and click the Download button next to Build Tools
for Visual Studio 2019.

Next, we will need to install the `rustup` tool, which allows us to very easily manage Rust toolchain installations as well
as easily update Rust when new versions are available. To download the tool, visit the website for the Rust programming language,
located [here](https://www.rust-lang.org/learn/get-started), or the Rustup website, located [here](https://rustup.rs/), and
select the 64-bit executable file to begin the process of initializing the Rustup utility.

#### With Windows Subsystem for Linux (WSL) 2

Installing Rust in the Windows Subsystem for Linux is even easier, and doesn't require Visual Studio 2019, or the Build Tools,
as the GNU Compiler Collection (gcc) is more than likely already installed for you. To install Rust, just run the following
command in a WSL terminal window and follow any instructions that are provided to you:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Rust may also be provided in the respective Windows Subsystem for Linux distribution you are using, however this is not recommended,
as the version of Rust available in the distribution's package repositories may be significantly outdated, due to the nature
of Ubuntu, Debian, and other non-rolling Linux distributions preferring to wait until new distribution versions to update
their packages to new major versions. For example, Ubuntu still has Rust 1.43.0 in their package repositories, a version
that was released in April of 2020, despite Rust 1.48.0 being the current stable version available, and installing Rust
through your system's package manager also removes the ability to have multiple Rust toolchains installed, which `rustup`
provides, among other features.

##### Advanced Notes

To install `rustup`, `rustc`, and `cargo` to a different folder than the default, create both the `RUSTUP_HOME` and the `CARGO_HOME`
system environment variables under the System Properties window in Windows, under Advanced. The `rustup` tool does not currently
offer a user-friendly way of changing the instal location, but this is an option if you would like to install Rust to e.g.,
a different drive.


### Installing the Bot

Now, we can actually download Jed and set him up. This step 100% requires Git, as that is how we will
be downloading him.

```bash
git clone https://github.com/Glowstick0017/JedBot.git
```

If you'd like to use GitLab for the cloning process instead of GitHub, you can do that too. Just use
the following command instead to clone from Jed's GitLab mirror.

```bash
git clone https://gitlab.com/Glowstick0017/JedBot.git
```



Now we can install JedBot's dependencies. On Windows, you will need to install the `windows-build-tools`
package using npm, as Windows does not natively include build tools like Linux does. For macOS, just
install Xcode and the commandline tools.

#### Non-release variant (unoptimized, with debug symbols)

```bash
cargo build
```

#### Release variant (optimized, without debug symbols)

```bash
cargo build --release
```

Just be patient while this process completes. It may take a while to complete, depending on your Internet
speed as well as the speed of your system's SSD and/or hard drive.


### Running the Bot

You have reached the final step of the install instructions. You're almost there. You just have to build
the bot and then start him up.

```bash
cargo run # (--release if you want to run the optimized variant)
```

Congratulations! You have (hopefully) successfully installed and set up JedBot, and you can now add the bot to
any guild you'd like. (if you have the permission to of course from Glowstick0017)

### Licensing 

MIT License

Copyright (c) 2021 Glowstick // Robbie

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

[invite-link]: https://discordapp.com/oauth2/authorize?client_id=907699386883112980&scope=bot
[invite-badge]: https://img.shields.io/badge/invite-to%20your%20Discord%20server-7289da.svg?style=flat-square&logo=discord

[dependency-link]: https://deps.rs/repo/github/Glowstick0017/JedBot
[dependency-badge]: https://deps.rs/repo/github/Glowstick0017/JedBot/status.svg


This project description was based off of <a href="https://github.com/KamranMackey/Ellie/blob/main/README.md">this</a> readme.md
