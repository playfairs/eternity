# Eternity

## Where will you spend _Eternity_?

Eternity is a local-first terminal application written in Rust. It is a longterm memory and reflection system for one person: questions, answers, journal entries, memories, time-locked capsules, and deterministic reflections over the life you record.

It is not a chatbot, a social network, a cloud service, an AI API wrapper, or a productivity app. Everything is stored locally in `data/eternity.db`.

## Features

- Ratatui/crossterm terminal interface
- SQLite persistence through bundled `rusqlite`
- JSON question banks in `.resources/questions`
- Question -> answer -> save -> reflection loop
- Journal, memory, capsule, profile, and timeline data model support
- Deterministic reflection engine using repeated words, themes, history, and topic frequency
- Time capsules that remain hidden until their unlock date

## Run

```sh
just run
```

Or directly:

```sh
cargo run
```

## Controls

- Type in the question screen to answer the current prompt
- `Enter` saves the answer and shows a reflection
- `n` moves to another question
- `h`, `q`, `r`, `j`, `c`, `p`, `t` switch screens
- `Esc` returns home
- `Ctrl-C` exits

## Development

```sh
just build
just test
just fmt
just clippy
```

The Nix flake provides a macOS/Linux development shell with Rust, SQLite, `pkg-config`, and `just`.

Nix layout:

- `nix/devShell.nix` defines `nix develop`
- `nix/buildPackage.nix` defines `nix build`
- `nix/formatter.nix` defines `nix fmt`