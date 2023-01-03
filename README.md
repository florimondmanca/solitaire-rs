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
 │  │▚▚▚│     │♥ 3│ │♥ K│ │♦ J│ │▚▚▚│ │▚▚▚│ │▚▚▚│ │▚▚▚│      │♠ A│  │
 │  │▚▚▚│     ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐      │  ♠│  │
 │  └───┘     │♥ 2│ │♣ Q│ │♦10│ │▚▚▚│ │▚▚▚│ │▚▚▚│ │▚▚▚│      └───┘  │
 │            │  ♥│ ┌───┐ ┌───┐ ┌───┐ │┌───┐┌───┐ ┌───┐      ┌───┐  │
 │  ┌───┐     └───┘ │♣ J│ │♥ 9│ │♥ 7│ └│♥ 5││▚▚▚│ │▚▚▚│      │♣ A│  │
 │  │♥ Q│           ┌───┐ ┌───┐ │┌───┐ ┌───┐┌───┐ │▚▚▚│      │  ♣│  │
 │  │  ♥│           │♣10│ │♥ 8│ └│♥ 6│ │♥ 4││▚▚▚│ └───┘      └───┘  │
 │  └───┘           │  ♣│ ┌───┐  │  ♥│ │  ♥│┌───┐            ┌───┐  │
 │                  └───┘ │♣ 7│  └───┘ └───┘│▚▚▚│            │♦ 2│  │
 │                        │  ♣│    ^        ┌───┐            │  ♦│  │
 │                        └───┘             │♦ 8│            └───┘  │
 │                                          ┌───┐            ┌╌╌╌┐  │
 │                                          │♠ 7│            ╎   ╎  │
 │                                          │  ♠│            ╎   ╎  │
 │                                          └───┘            └╌╌╌┘  │
 │                                                                  │
 └──────────────────────────────────────────────────────────────────┘
```

## Credits

This software wouldn't be possible without the following crates:

* [termion](https://docs.rs/termion)
* [tui-rs](https://github.com/fdehau/tui-rs)

## License

MIT
