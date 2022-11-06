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
| `$` | pop 1 bit off the fornt of the queue |
| `.` | output the bit at the front of the queue |
| `,` | get the next bit of input and push it to the back of the queue |
| `[if_num\|else_num]` | see note |

# Turing Completeness
CTFuck is turing complete. To prove this, we can take a similar path to how [BCT](https://esolangs.org/wiki/Bitwise_Cyclic_Tag) was proved turing complete: we can translate each CT (we are using the same language the BCT page used) command to an equivalent sequence of commands in CTFuck.

| CT command | CTF Equivalent |
| ---------- | --------------- |
| \<program start\> | `@` |
| \<program end\> | `[1|1]` |
| `0` | `0` |
| `1` | `1` |
| `;` | `@$[\|<next @'s rank>]` |

where the nth `@`'s rank is defined as n.

# Notes
## The `[if_num|else_num]` command
it takes two numbers in bijective base-10 format, separated by `|` (pipe symbol).

when it is executed, it checks the bit at the top of the queue;
if it is 1 it calls `goto(if_num)`, otherwise it calls `goto(else_num)`.

where `goto(n)` is defined as follows:
    if n is empty, (meaning this command is of the form `[|<non-empty>]`, `[<non-empty>|]`, or `[|]`), just go to the next command.
    otherwise jump to the `n`th `@` in the program.

## IO is big endian
Unlike BoolFuck, IO in CTF is big-endian, which means if you do `,,` and input something like `@`, (`0100 0000` in binary), then unlike Boolfuck, you'd first push `0` and then `1`. output would behave the same way: `0.1.0......` would print `@` to stdout.
