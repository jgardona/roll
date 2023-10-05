# Roll
Roll is a cli to help people who plays Dangeons and Dragons. It implements many dice types: d4, d6, d8, d10, d20.

### Install

```
$ cargo install --git https://github.com/jgardona/roll
```

### Use

```
$ ./roll -h
This command exists to help roll dices in D&D games.
The following are implemented: d4, d6, d8, d10, d20.

Usage: roll <DICES> <AMOUNT>

Arguments:
  <DICES>   The type of dice to roll [possible values: d4, d6, d8, d10, d20]
  <AMOUNT>  The amount of dices to roll

Options:
  -h, --help     Print help
  -V, --version  Print version

$ ./roll d4 3

Processing your results

⚄  1,1 ⚄  2,1 ⚄  3,1 

Total of rolls: 3
```