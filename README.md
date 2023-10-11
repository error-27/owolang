# owolang
A very silly esolang :3

## Syntax
All instructions are made up of two separate tokens, with a `w` in between, with each token meant to represent an eye in an OwO-like face. The first eye is an action, and the second is a motion, which defines what the action will apply to, similar to Vim commands. Strings are entered through a separate string mode, in which the next tokens are interpreted as a character and a motion.

## Data
All data is stored in an infinite memory strip of 8-bit unsigned integers, just like with brainfuck. A pointer keeps track of what cell you are currently selecting. Some motions may act on cells outside of the currently selected one.

## Tokens
### Actions

### Motions
| Token | Effect                                  |
|-------|-----------------------------------------|
| `O`   | Apply action to currently selected cell |