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
┌───┐  ┌╌╌╌┐         ┌╌╌╌┐  ┌╌╌╌┐  ┌╌╌╌┐  ┌╌╌╌┐
│▚▚▚│  ╎   ╎         ╎   ╎  ╎   ╎  ╎   ╎  ╎   ╎
│▚▚▚│  ╎   ╎         ╎   ╎  ╎   ╎  ╎   ╎  ╎   ╎
└───┘  └╌╌╌┘         └╌╌╌┘  └╌╌╌┘  └╌╌╌┘  └╌╌╌┘

┌───┐  ┌───┐  ┌───┐  ┌───┐  ┌───┐  ┌───┐  ┌───┐
│♠ 9│  │▚▚▚│  │▚▚▚│  │▚▚▚│  │▚▚▚│  │▚▚▚│  │▚▚▚│
│ ♠ │  ┌───┐  ┌───┐  ┌───┐  ┌───┐  ┌───┐  ┌───┐
└───┘  │♥ K│  │▚▚▚│  │▚▚▚│  │▚▚▚│  │▚▚▚│  │▚▚▚│
       │ ♥ │  ┌───┐  ┌───┐  ┌───┐  ┌───┐  ┌───┐
       └───┘  │♦ 7│  │▚▚▚│  │▚▚▚│  │▚▚▚│  │▚▚▚│
              │ ♦ │  ┌───┐  ┌───┐  ┌───┐  ┌───┐
              └───┘  │♠ 2│  │▚▚▚│  │▚▚▚│  │▚▚▚│
                     │ ♠ │  ┌───┐  ┌───┐  ┌───┐
                     └───┘  │♥ J│  │▚▚▚│  │▚▚▚│
                            │ ♥ │  ┌───┐  ┌───┐
                            └───┘  │♠ 7│  │▚▚▚│
                                   │ ♠ │  ┌───┐
                                   └───┘  │♦ 5│
                                          │ ♦ │
                                          └───┘
Hint: 'q' will exit
```

## Credits

This software wouldn't be possible without the following tools and libraries:

* [termion](https://docs.rs/termion)

## License

MIT
