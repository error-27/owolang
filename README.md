# owolang
A very silly esolang :3

## Syntax
All instructions are made up of two separate tokens, with a `w` in between, with each token meant to represent an eye in an OwO-like face. The first eye is an action, and the second is a motion, which defines what the action will apply to, similar to Vim commands. Strings are entered through a separate string mode, in which the next tokens are interpreted as a character and a motion.

## Data
All data is stored in an infinite memory strip of 8-bit unsigned integers, just like with brainfuck. A pointer keeps track of what cell you are currently selecting. Some motions may act on cells outside of the currently selected one.

## Strings
Strings are created by entering **String Mode**. String Mode starts with a timer that determines how long until String Mode ends. It counts down every time an instruction is read, and is initially set to the value referenced by the Motion when String Mode starts. While in String Mode all Action tokens are taken as a character to add to the string, and are added to the string the same number of times as the number in the cell referenced by the Motion. There is only ever one string, and it gets reset when string mode starts.

## Tokens
### Actions
| Token | Effect                                  |
|-------|-----------------------------------------|
| `O`   | Select cell |
| `^`   | Increment cell |
| `-`   | Decrement cell |
| `U`   | Start string mode |
| `V`   | Print out The String the number of times in the referenced cell |

### Motions
| Token | Effect                                  |
|-------|-----------------------------------------|
| `U`   | Apply action to currently selected cell |
| `O`   | Apply action to next cell |
| `Q`   | Apply action to previous cell |