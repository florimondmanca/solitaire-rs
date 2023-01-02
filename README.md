# solitaire-rs

Play [Solitaire](https://www.officialgamerules.org/solitaire) in the terminal. Written in [Rust](https://www.rust-lang.org/).

## Prerequisites

* Rust 1.62+ (hint: install using [rustup](https://rustup.rs/))

## Quickstart

Build and run:

```
make
```

Run code formatting:

```
make format
```

## Preview

Terminal UI (TUI):

```
┌Solitaire─────────────────────────────────────────────────────────┐
│  ┌───┐     ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐      ┌───┐  │
│  │▚▚▚│     │♠ 2│ │♣ K│ │▚▚▚│ │▚▚▚│ │▚▚▚│ │▚▚▚│ │▚▚▚│      │♥ 2│  │
│  │▚▚▚│     │  ♠│ │  ♣│ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐      │  ♥│  │
│  └───┘     └───┘ └───┘ │▚▚▚│ │▚▚▚│ │▚▚▚│ │▚▚▚│ │▚▚▚│      └───┘  │
│                        ┌───┐ │┌───┐┌───┐ ┌───┐ ┌───┐      ┌───┐  │
│  ┌───┐                 │♦ K│ └│♠ 6││♠ 8│ │▚▚▚│ │▚▚▚│      │♦ A│  │
│  │♠ 4│                 ┌───┐  │  ♠││  ♠│ ┌───┐ ┌───┐      │  ♦│  │
│  │  ♠│                 │♥ Q│  └───┘└───┘ │▚▚▚│ │▚▚▚│      └───┘  │
│  └───┘                 │  ♥│    ^        ┌───┐ ┌───┐      ┌╌╌╌┐  │
│                        └───┘             │▚▚▚│ │▚▚▚│      ╎   ╎  │
│                                          │▚▚▚│ ┌───┐      ╎   ╎  │
│                                          └───┘ │▚▚▚│      └╌╌╌┘  │
│                                                ┌───┐      ┌╌╌╌┐  │
│                                                │♣ 3│      ╎   ╎  │
│                                                ┌───┐      ╎   ╎  │
│                                                │♦ 2│      └╌╌╌┘  │
│                                                │  ♦│             │
│                                                └───┘             │
└──────────────────────────────────────────────────────────────────┘
```

## Credits

This software wouldn't be possible without the following crates:

* [termion](https://docs.rs/termion)
* [tui-rs](https://github.com/fdehau/tui-rs)

## License

MIT
