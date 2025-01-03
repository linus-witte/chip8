# CHIP-8 Emulator

An implementation of a CHIP-8 emulator written in rust. 

## Running the Emulator

```bash
cargo run -- path/to/rom.ch8
```

## Keymap

The CHIP-8 keypad layout is as follows:

```
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```

It maps to the following keys on the keyboard (left side of the colemak layout):

```
1 2 3 4
Q W F P
A R S T
X C D V
```
