# checkline: checkbox line picker for stdin line input

Example:

```sh
printf "%s\n%s\n%s\n" alpha bravo charlie > example.txt
cat example.txt | checkline
```

You should see each line with a checkbox and text:

```txt
[ ] alpha
[ ] bravo
[ ] charlie
```

Move up and down in the list by typing arrow keys.

Check or uncheck any checkbox by typing space or return.

Check each line that you want, then type ESC to escape.

The command prints each line you want.

## Install

Install checkline as a typical Rust crate:

```sh
cargo install checkline
```

If people want other ways, such as with package managers, we welcome help to create these ways.


## Purpose

The purpose of this command is a simple picker, that is easy to use, and
that work wells in on the command line such as within a pipe.

The purpose isn't intended to handle very long lines, or very long inputs.


## Projects with similarities

`vipe` that can pipe in and out of `$EDITOR`:
<https://github.com/juliangruber/vipe>

`peco` simplistic interactive filtering tool:
<https://github.com/peco/peco>

`percol` adds interactive selection to the traditional pipe concept.
<https://github.com/mooz/percol>

`canything` interactive grep tools:
<https://github.com/keiji0/canything>

`zaw` zsh-friendly interactive grep tool:
<https://github.com/zsh-users/zaw>

`fzf` interactive grep tool written in Go language.
<https://github.com/junegunn/fzf>


## Feedback

We welcome consructive criticism and ideas for improvements.


## Tracking

* Program: checkline
* Version: 1.1.0
* License: MIT OR BSD OR GPL-2.0 OR GPL-3.0
* Created: 2022-10-15T12:24:50Z
* Updated: 2022-10-17T21:13:13Z
* Website: https://github.com/sixarm/checkline
* Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)
