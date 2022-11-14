# CTFuck
a turing complete esolang based on a tag system, with I/O support.
this is kinda simliar to Boolfuck, in that it operates on bits and has I/O support.
however, CTFuck is based on a tag system.

# Commands

supported commands are:
| commands | operation |
| -------- | --------- |
| `0` | push `0` to the back of queue |
| `1` | push `1` to the back of queue |
| `$` | pop 1 bit off the front of the queue |
| `.` | output the bit at the front of the queue |
| `,` | get the next bit of input and push it to the back of the queue |
| `[if_num\|else_num]` | see [note](#the-if_numelse_num-command) |

any character other than these are ignored.

# Turing Completeness
CTFuck is turing complete. To prove this, we can take a similar path to how [BCT](https://esolangs.org/wiki/Bitwise_Cyclic_Tag) was proved turing complete: we can translate each CT (we are using the same language the BCT page used) command to an equivalent sequence of commands in CTFuck.

| CT command | CTF Equivalent |
| ---------- | --------------- |
| \<program end\> | `\n[1\|1]` |
| `0` | `0` |
| `1` | `1` |
| `;` | `\n$[\|<current line + 1>]` |

# Notes
## The `[if_num|else_num]` command
it takes two numbers in base-10 format, separated by `|` (pipe symbol).

when it is executed, it checks the bit at the top of the queue;
if it is 1 it calls `goto(if_num)`, otherwise it calls `goto(else_num)`.

`goto(n)` does the following: if `n` is 0 or empty it does nothing, otherwise it jumps to the first command in line `n`.

## IO endianness
Like BoolFuck, IO in CTF is little-endian.

## efficiency

while designing a [boolfuck to ctfuck compiler](https://github.com/pro465/btcc), i found that CTFuck is as efficient as a Clockwise Turing Machine. but that does not outdate the name, as long as we ignore the M in Machine...
