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


## Purpose

The purpose of this command is a simple picker, that is easy to use, and
that work wells in on the command line such as within a pipe.

The purpose isn't intended to handle very long lines, or very long inputs.


## Comparisons

See the command `vipe` that can pipe in and out of `$EDITOR`:
<https://github.com/juliangruber/vipe>


## Feedback

We welcome consructive criticism and ideas for improvements.


## Tracking

* Program: checkline
* Version: 1.0.0
* License: MIT OR BSD OR GPL-2.0 OR GPL-3.0
* Created: 2022-10-15T12:24:50Z
* Updated: 2022-10-17T21:13:13Z
* Website: https://github.com/sixarm/checkline
* Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)
