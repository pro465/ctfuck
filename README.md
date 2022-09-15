# tagfuck
a turing complete esolang based on a tag system, with I/O support.
this is kinda simliar to Boolfuck, in that it operates on bits and has I/O support.
however, Tagfuck is based on a tag system.

# Commands

supported commands are:
| commands | operation |
| -------- | --------- |
| `?` | if value at front of queue is `false`, skip the next command |
| `#` | skip next command |
| `/` | push `true` to the back of queue |
| `\` | push `false` to the back of queue |
| `[` | pop value off the front of queue |
| `]` | always jump to the command just after the matching `]` |
| `,` | get the next bit of input and push it to the back of the queue |
| `.` | output the bit the front of the queue |

# Turing Completeness
tagfuck is turing complete. To prove this, we can take a similar path to how [BCT](https://esolangs.org/wiki/Bitwise_Cyclic_Tag) was proved turing complete: we can translate each CT (we are using the same language the BCT page used) command to an equivalent sequence of commands in Tagfuck.

| CT command | Tagf Equivalent |
| ---------- | --------------- |
| \<program start\> | `#[` |
| \<program end\> | `]` |
| `0` | `?\` |
| `1` | `?/` |
| `;` | `[#]` |
