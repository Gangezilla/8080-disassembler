// -1: pass in which file X
// 0: get the data from the file X
// 0.5: put it in the appropriate format X
// 1: Read the code into a buffer X
// 2: Get a pointer to the beginning of the buffer
// 3: Use the byte at the pointer to determine the opcode
// 4: Print out the name of the opcode using the bytes after the opcode as data, if applicable
// 5: Advance the pointer the number of bytes used by that instruction (1, 2, or 3 bytes)
// 6: If not at the end of the buffer, go to step 3

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut position = 0;
    let end_position = buffer.len();
    while position < end_position {
        let hex = &format!("{:x}", buffer[position]);
        match &hex as &str {
            "0" => {
                println!("00: NOP");
                position += 1;
            }
            "f" => {
                // The content of the accumulator is rotated right one position.
                // The high order bit and the CY flag are both set to the value shifted out of the low order bit position. Only the CY flag is affected.
                println!("0f: RRC");
                position += 1;
            }
            "21" => {
                // Load immediate register
                // LXI rp, data 16
                // Byte 3 of the instruction is moved into the high order register of the register pair rp.
                // byte 2 of the instruction is moved into the low order register (rl) of the restier pair rp.
                println!(
                    "21: LXI H,D16. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "23" => {
                // Increment H & L registers
                println!("23: INX H");
                position += 1;
            }
            "27" => {
                // Decimal Adjust A(ccumulator)
                // The eight-bit number in the accumulator is adjusted to form two four-bit Binary-Coded-Decimal digits by the following process:
                // If the value of the least significant 4 bits of the accumulator is greater than 9 or if the AC flag is set, 6 is added to the accumulator.
                // If the value of the most significant 4 bits of the accumulator is now greater than 9, or if the CY flag is set, 6 is added to the most significant 4 bits of the accumulator.
                println!("27: DAA");
                position += 1;
            }
            "32" => {
                // Store A direct.
                // The content of the accumulator is moved to the next two bytes.
                println!(
                    "32: STA {:x}, {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "35" => {
                // Decrements Register M.
                // The content of the memory location whose address is contained in the Hand L registers is decremented by one.
                // Sets the flags 	Z, S, P, AC
                println!("35: DCR M");
                position += 1;
            }
            "66" => {}
            "7e" => {
                // Move memory to register
                println!("7e: MOV A,M");
                position += 1;
            }
            "3a" => {
                // Load Accumulator Direct
                // The content of the memory location, whose address is specified in byte 2 and byte 3 of the instruction, is moved to register A.
                println!(
                    "3a: LDA adr. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3
            }
            "3e" => {
                // Move immediate register
                println!("3e: MVI A,D8 {:x}", buffer[position + 1]);
                position += 2;
            }
            "a7" => {
                // AND register
                // The content of register r is logically anded with the content of the accumulator.
                // The result is placed in the accumulator. The CY flag is cleared.
                println!("a7: ANA A");
                position += 1;
            }
            "af" => {
                // Exclusive OR Register
                // The content of register r is exclusive-or'd with the content of the accumulator. The result is placed in the accumulator. The CY and AC flags are cleared.
                println!("af: XRA A");
                position += 1;
            }
            "c1" => {
                // Pop register pair B & C off stack
                println!("c1: POP B");
                position += 1;
            }
            "c2" => {
                // Jump on no zero.
                // not sure, but i think its jump if a check is non zero.
                println!(
                    "c2: JNZ adr. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "c3" => {
                // JMP, uses next two bytes to determine where to jump to.
                println!(
                    "c3: JMP: {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "c5" => {
                // Push register Pair B & C on stack
                println!("c5: PUSH B");
                position += 1;
            }
            "c6" => {
                // Add immediate to A.
                // The content of the second byte of the instruction is added to the content of the accumulator.
                // The result is placed in the accumulator.
                println!("c6: ADI D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "c9" => {
                // Return
                // The content of the memory location whose address is specified in register SP is moved to the low-order eight bits of register PC.
                // The content of the memory location whose address is one more than the content of register SP is moved to the high-order eight bits of register PC. The content of register SP is incremented by 2.
                println!("c9: RET");
                position += 1;
            }
            "ca" => {
                // Jump on zero
                // not sure. i imagine it checks to see if something is 0, then jumps...
                println!(
                    "ca: JZ adr. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "cd" => {
                // The high-order eight bits of the next instruction address are moved to the memory location whose address is one less than the content of register SP.
                // The low-order eight bits of the next instruction address are moved to the memory location whose address is two less than the content of register SP.
                // The content of register SP is decremented by 2. Control is transferred to the instruction whose address is specified in byte 3 and byte 2 of the current instruction.
                println!(
                    "cd: CALL adr. {:x}, {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }

            "d1" => {
                // Pop register pair D & E off stack
                println!("d1: POP D");
                position += 1;
            }
            "d2" => {
                // Jump if no carry
                // not sure, but i imagine it's jump if the no carry flag hasn't been set.
                println!(
                    "d2: JNC. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3
            }
            "d5" => {
                // Push register Pair D & E on stack
                println!("d5: PUSH D");
                position += 1;
            }
            "da" => {
                // Conditional Jump
                // If the specified condition is true, control is transferred to the instruction whose address is specified in byte 3 and byte 2 of the current instruction; other- wise, control continues sequentially.
                println!(
                    "da: JC adr. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "db" => {
                // IN port
                // The data placed on the eight bit bi-directional data bus by the specified port is moved to register A.
                println!("db: IN d8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "e1" => {
                // Pop register pair H & L off stack
                println!("e1: POP H");
                position += 1;
            }
            "e5" => {
                // Push register Pair H & L on stack
                println!("e5: PUSH H");
                position += 1;
            }
            "f1" => {
                // Pop A and Flags off stack
                println!("f1: POP PSW");
                position += 1;
            }
            "f5" => {
                // PSW means "processor state word", adds 1 to accumulator.
                // Push A and Flags on stack
                println!("f5: PUSH PSW");
                position += 1;
            }
            "fb" => {
                // Enable interrupts
                // The interrupt system is enabled following the execu- tion of the next instruction.
                println!("fb: EI");
                position += 1;
            }
            "fe" => {
                // Compare immediate
                // The content of the second byte of the instruction is subtracted from the accumulator.
                // The condition flags are set by the result of the subtraction.
                // The Z flag is set to 1 if (A) = (byte 2). The CY flag is set to 1 if (A) <(byte 2).
                println!("fe: CPI D8. {:x}", buffer[position + 1],);
                position += 2;
            }
            _ => {
                println!("Unimplemented instruction: {}", hex);
                position += 9999;
            }
        }
    }
    // for i in buffer.iter() {
    // }
}

