# ctfuck
a turing complete esolang based on a tag system, with I/O support.
this is kinda simliar to Boolfuck, in that it operates on bits and has I/O support.
however, Tagfuck is based on a tag system.

# Commands

supported commands are:
| commands | operation |
| -------- | --------- |
| `#` | skip next command |
| `0` | push `0` to the back of queue |
| `1` | push `1` to the back of queue |
| `[` | pop 1 bit off the front of queue |
| `]` | jump to the command just after the matching `[` |
| `?` | if value at front of queue is `0`, skip the next command |
| `.` | output the bit at the front of the queue |
| `,` | get the next bit of input and push it to the back of the queue |

# Turing Completeness
CTFuck is turing complete. To prove this, we can take a similar path to how [BCT](https://esolangs.org/wiki/Bitwise_Cyclic_Tag) was proved turing complete: we can translate each CT (we are using the same language the BCT page used) command to an equivalent sequence of commands in Tagfuck.

| CT command | CTF Equivalent |
| ---------- | --------------- |
| \<program start\> | `#[` |
| \<program end\> | `]` |
| `0` | `?0` |
| `1` | `?1` |
| `;` | `[#]` |

# The `#` command
Technically, I could do with just a separate pop command rather than integrating it into `[` (then I'd not need `#` to prove it's turing complete), but I chose to do what I did because I found that `#` interacts nicely with other commands in atleast 3 convenient ways (that I could find):
 1. `[#]` brings back the pop command.
 2. `#[<code>]` brings back the "loop without popping" command.
 3. added bonus: `?#` works as an "if not" command.

So, it allows what I'd get with a separate pop command, as well as introducing a new "if not" command. And that's just what I **_could_** find...
