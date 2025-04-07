# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## How to Run
Clone the repo, navigate to the root folder, then execute:
```npm run tauri dev```

## Project Description
I want to build a desktop music player that is more friendly to organizing classical music. I have large collection that I inherited from a family member, and I have not found a music player that works for me in keeping it organized and accessible.

I also am very interested in learning Rust, so using Tauri as the framework will allow me to do that along the way.

I'm trying to do as much of this from scratch as possible because it's more of a learning exercise then a rush to make a productive application. That's why I'm not using any third-party UI libraries or anything like that.

## Tools I'm Using
[Tauri](https://tauri.app/) - Overall framework. Rust-based back-end + React/TypeScript front-end.
### Rust Crates
[rodio](https://docs.rs/rodio/latest/rodio/) to decode and play audio files.
<br>
[diesel](https://diesel.rs/) to manage and query a SQLite database.
<br>
[audiotags](https://docs.rs/audiotags/latest/audiotags/) to read metadata from music files.
<br>
[dotenv](https://docs.rs/dotenv/latest/dotenv/) to read configuration from local .env file.
### Front-end tools
[React Router](https://reactrouter.com/) to navigate between screens.
<br>
[mobx](https://github.com/mobxjs/mobx) to manage global state.
<br>
[vitest](https://vitest.dev/guide/) for automated testing.


