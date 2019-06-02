# 8080-disassembler
Disassembler for compiled code that runs on the 8080 processor.

Run `cargo run path/to/file` and you'll get a nice disassembled view of all the opcodes in the file.

Interpret your output like this:

`1fe9     11: LXI D #26 12` = `Instruction number (in hex)      Op Code (in hex): Instruction #bytes being operated on
