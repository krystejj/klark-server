<img src="./.github/assets/klark-banner-275x120.png" alt="Klark banner" />

# Klark Server
A modern and feature-rich media server and collection manager. Part of **The Klark Project**.

![License](https://img.shields.io/github/license/krystejj/klark-server?label=License)
![Latest Release](https://img.shields.io/github/v/release/krystejj/klark-server?label=Latest%20Release)
![Issues](https://img.shields.io/github/issues/krystejj/klark-server?label=Issues)
![Pull Requests](https://img.shields.io/github/issues-pr/krystejj/klark-server?label=Pull%20Requests)
![Discussions](https://img.shields.io/github/discussions/krystejj/klark-server?label=Discussions)

> [!CAUTION]
> **This software is in VERY, and I really mean VERY early stage of development!** It still doesn't have the most basic features and could even not work at all. It is basically unusable for now, because there isn't even what to use at this stage.

> [!WARNING]
> **Be prepared for a firestorm!** I ([krystejj](https://github.com/krystejj)) am still learning to code in Rust. That means there can be major and really basic bugs in my code. While I am expanding my experience with Rust and learning new things, be aware that you may find even most basic mistakes here. You can always create a pull request or issue in this repo when you find or encounter any error, all help is always appreciated.

**Klark** is a free, modern and open-source media server and collection manager, as a part of **The Klark Project**. It hosts an API which frontends can use, it also can host a web frontend if you like. Klark allows you to easily watch your collected movies, tv series or photos, listen to your music and read your ebooks, manage them, search and download new using BitTorrent, and all of this from any device you want through the client. Klark also aims to be very fast, performant and easy to use.

## 💻 Clients / Frontends
Currently only one main client is in development, that is the [Klark Svelte Frontend](https://github.com/krystejj/klark-svelte-frontend). However, other clients like Android or Apple TV apps are also planned, but web frontend is one with highest priority for now because of very early stage of development. You can create a new client on your own if you like.

There are also plans for [Jellyfin](https://jellyfin.org/) protocol support. That means you could use any Jellyfin client like [Jellyfin Android](https://github.com/jellyfin/jellyfin-android) or [Swiftfin](https://github.com/jellyfin/swiftfin) with your Klark server. Other protocols might be supported too.

In development:
- [Klark Svelte Frontend](https://github.com/krystejj/klark-svelte-frontend) - web, main

## 📖 Project Management
All notable changes to this project will be documented in the [changelog file](CHANGELOG.md). The format of that file is based on [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/).

This project adheres to [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html).

You can find this project backlog in [projects tab](https://github.com/users/krystejj/projects/4/views/1) in this repo.

If you would find any bug, some type of issue or you have an idea for improving a project, you can file a report using [issues page](https://github.com/krystejj/klark-server/issues) in this repo or create a [pull request](https://github.com/krystejj/klark-server/pulls). [Discussions](https://github.com/krystejj/klark-server/discussions) are also available.

Information about security lies in [security policy file](SECURITY.md).

## 🙏 Used Projects and Credits
This is a list of projects used in development of this project:
- [Rust](https://www.rust-lang.org/) - an incredibly fast and reliable programming language that guarantees memory and thread safety.
- [Actix](https://actix.rs/) - a powerful and extremely fast web framework for Rust.

💗 Big thanks to the creators and all contributors of these projects.

## 📜 License
This project is provided under the terms of the **GNU General Public License v3.0**, a free and open-source license. For more information, see the [license file](LICENSE.md).
