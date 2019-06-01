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
            "1" => {
                // Load byte 3 into register B. Load byte 2 into register C.
                println!(
                    "01: LXI B,D16. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "4" => {
                // Increment B
                println!("04: INR B");
                position += 1;
            }
            "5" => {
                // Decrement B
                println!("05: DCR B");
                position += 1;
            }
            "6" => {
                // Move immediate second byte to register B
                println!("06: MVI B, D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "7" => {
                // Rotate left.
                // The content of the accumulator is rotated left one position.
                // The low order bit and the CY flag are both set to the value shifted out of the high order bit position.
                // Only the CY flag is affected.
                println!("07: RLC");
                position += 1;
            }
            "a" => {
                // Load B indirect
                println!("0a: LDAX B");
                position += 1;
            }
            "d" => {
                // Decrement C.
                println!("0d: DCR C");
                position += 1;
            }
            "e" => {
                // Move byte 2 into register C.
                // MVI stands for Move Immediate.
                println!("0e: MVI C,D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "f" => {
                // The content of the accumulator is rotated right one position.
                // The high order bit and the CY flag are both set to the value shifted out of the low order bit position. Only the CY flag is affected.
                println!("0f: RRC");
                position += 1;
            }
            "11" => {
                // Move byte 3 into register D. Move byte 2 into register E.
                // LXI stands for Load Immediate Register
                println!(
                    "11: LXI D,D16. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "13" => {
                // Increment register DE
                println!("13: INX D");
                position += 1;
            }
            "14" => {
                // Increment Register D
                println!("14: INR D");
                position += 1;
            }
            "15" => {
                // Decrement D
                println!("15: DCR D");
                position += 1;
            }
            "16" => {
                // Move byte 2 into D
                println!("16: MVI D, D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "19" => {
                // Add D & E to H & L
                println!("19: DAD D");
                position += 1;
            }
            "1a" => {
                // Load A indirect
                // The content of the memory location, whose address is in the register pair rp, is moved to register A.
                // Note: only register pairs rp=B (registers B and C·) or rp=D (registers D and E) may be specified.
                println!("1a: LDAX D");
                position += 1;
            }
            "20" => {
                println!("20: -");
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
            "22" => {
                // Store H & L direct
                // The content of register L is moved to the memory location whose address is specified in byte 2 and byte 3.
                // The content of register H is moved to the next memory location.
                // ((byte 3) (byte 2)) <- (L)
                // ((byte 3)(byte 2) + 1) <- (H)
                println!(
                    "22: SHLD adr. {:x} {:x}",
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
            "26" => {
                // Move byte into register H.
                println!("26: MVI H,D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "27" => {
                // Decimal Adjust A(ccumulator)
                // The eight-bit number in the accumulator is adjusted to form two four-bit Binary-Coded-Decimal digits by the following process:
                // If the value of the least significant 4 bits of the accumulator is greater than 9 or if the AC flag is set, 6 is added to the accumulator.
                // If the value of the most significant 4 bits of the accumulator is now greater than 9, or if the CY flag is set, 6 is added to the most significant 4 bits of the accumulator.
                println!("27: DAA");
                position += 1;
            }
            "29" => {
                // Adds HI to HL.
                println!("29: DAD H");
                position += 1;
            }
            "2a" => {
                // Load H & L direct
                // The content of the memory location, whose address is specified in byte 2 and byte 3 of the instruction, is moved to register L.
                //  The content of the memory location at the succeeding address is moved to register H.
                println!(
                    "2a: LHLD addr. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
            }
            "2b" => {
                // Decrement H & L
                println!("2b: DCX H");
                position += 1;
            }
            "2c" => {
                // Increment Register L
                println!("2c: INR L");
                position += 1;
            }
            "2e" => {
                // Move byte 2 into register L
                println!("2e: MVI L,D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "31" => {
                // move byte 3 into high order location of register SP. move byte 2 into low order location of register SP.
                println!(
                    "31: LXI SP,D16. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
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
            "34" => {
                // Increment Register M (HL)
                println!("34: INR M");
                position += 1;
            }
            "35" => {
                // Decrements Register M.
                // The content of the memory location whose address is contained in the H and L registers is decremented by one.
                // Sets the flags 	Z, S, P, AC
                println!("35: DCR M");
                position += 1;
            }
            "36" => {
                // Move second byte, into register HL.
                println!("36: MVI M,D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "37" => {
                // Set carry
                // Sets carry flag to 1
                println!("37: STC");
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
            "3c" => {
                // Increment Register A
                println!("3c: INR A");
                position += 1;
            }
            "3d" => {
                // Decrement Register A
                println!("3d: DCR A");
                position += 1;
            }
            "3e" => {
                // Move immediate register
                println!("3e: MVI A,D8 {:x}", buffer[position + 1]);
                position += 2;
            }
            "46" => {
                // Move the contents of HL to B
                println!("46: MOV B,M");
                position += 1;
            }
            "47" => {
                // Move contents of A to B.
                println!("47: MOV B,A");
                position += 1
            }
            "4e" => {
                // Move HL to C
                println!("4e: MOV C,M");
                position += 1;
            }
            "4f" => {
                // Move A to C.
                println!("4f: MOV C,A");
                position += 1;
            }
            "56" => {
                // Move register HL into register D.
                println!("56: MOV D,M");
                position += 1;
            }
            "5e" => {
                // Move register HL to register E.
                println!("5e: MOV E,M");
                position += 1;
            }
            "5f" => {
                // Move contents of A to E.
                println!("5f: MOV E,A");
                position += 1;
            }
            "61" => {
                // Move contents of register C to register H
                println!("61: MOV H,C");
                position += 1;
            }
            "66" => {
                // Move memory (HL) to register H
                // H <- (HL)
                println!("66: MOV H,M");
                position += 1;
            }
            "67" => {
                // Move register A to register H
                println!("67: MOV H,A");
                position += 1;
            }
            "68" => {
                // Move register B to register A.
                println!("68: MOV L,B");
                position += 1;
            }
            "69" => {
                // Move value of C into register L.
                println!("69: MOV L,C");
                position += 1;
            }
            "6f" => {
                // Move memory (L) to register A
                // L <- A
                println!("6f: MOV L,A");
                position += 1;
            }
            "70" => {
                // Move register B to register M (HL);
                println!("70: MOV M,B");
                position += 1;
            }
            "71" => {
                // Move register C into register M.
                println!("71: MOV M,C");
                position += 1;
            }
            "72" => {
                // Move register D into register M(HL);
                println!("72: MOV M,D");
                position += 1;
            }
            "73" => {
                // Move register E to register M (HL)
                println!("73: MOV M,E");
                position += 1;
            }
            "77" => {
                // Move register A to register HL.
                println!("77: MOV M,A");
                position += 1;
            }
            "78" => {
                // Move contents of register B to register A.
                println!("78: MOV A,B");
                position += 1;
            }
            "79" => {
                // Move register C to A.
                println!("79: MOV A,C");
                position += 1;
            }
            "7a" => {
                // Move contents of register D to register A
                println!("7a: MOV A,D");
                position += 1;
            }
            "7b" => {
                // Move contents of E to A
                println!("7b: MOV A,E");
                position += 1;
            }
            "7c" => {
                // Move register H into register A.
                println!("7c: MOV A,H");
                position += 1;
            }
            "7d" => {
                // Move contents of L to A
                println!("7d: MOV A,L");
                position += 1;
            }
            "7e" => {
                // Move memory to register
                println!("7e: MOV A,M");
                position += 1;
            }
            "80" => {
                // Add value of B to the accumulator to A
                println!("80: ADD B");
                position += 1;
            }
            "85" => {
                // Adds register L to register A.
                println!("85: ADD L");
                position += 1;
            }
            "86" => {
                // Add M (HL) to register A.
                println!("86: ADD M");
                position += 1;
            }
            "97" => {
                // Subtract value of A from A (i guess sets A to 0)
                println!("97: SUB A");
                position += 1;
            }
            "a0" => {
                // Does an and of A & B on A
                println!("a0: ANA B");
                position += 1;
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
            "b" => {
                // Decrement Register BC
                println!("0b: DCX B");
                position += 1;
            }
            "b0" => {
                // A <- A | B
                // Or register with A.
                // Does an OR with register B on register A.
                println!("b0: ORA B");
                position += 1;
            }
            "b4" => {
                // A <- A | H
                // Or register H with register A.
                println!("b4: ORA H");
                position += 1;
            }
            "b8" => {
                // Compare register B to A.
                // Contents of register B are substracted from A.
                // couldnt tell you why its called compare
                println!("b8: CMP B");
                position += 1;
            }
            "be" => {
                // A - (HL)
                // Subtract HL from A.
                println!("be: CMP M");
                position += 1;
            }
            "c0" => {
                // If NZ (non-zero), RET (return).
                println!("c0: RNZ");
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
            "c4" => {
                // Call address if non zero
                // Not sure what it checks? whats non zero
                println!(
                    "c4: CNZ adr. {:x} {:x}",
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
            "c8" => {
                // 	If Z, RET
                // if what is zero??
                println!("c8: RZ");
                position += 1;
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
            "cc" => {
                // If Z, call adr
                // Z is one of the flags. Must check if that flag is set then calls.
                println!(
                    "cc: CZ adr. {:x} {:x}",
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
            "d0" => {
                // 	if NCY, RET
                // return on no carry. if the carry flag hasn't been set, return.
                println!("d0: RNC");
                position += 1;
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
            "d3" => {
                // Output
                // The content of register A is placed on the eight bit bi-directional data bus for transmission to the specified port.
                println!("d3: OUT D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "d5" => {
                // Push register Pair D & E on stack
                println!("d5: PUSH D");
                position += 1;
            }
            "d6" => {
                // Subtract immediate from A
                // Subtracts the byte value from A.
                println!("d6: SUI D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "d8" => {
                // if carry flag is set, return.
                println!("d8: RC");
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
            "de" => {
                // Subtract immediate from A with borrow
                // The contents of the second byte of the instruction and the contents of the CY flag are both subtracted from the accumulator.
                // The result is placed in the accumulator.
                println!("de: SPI d8. {:x}", buffer[position + 1]);
                position += 1;
            }
            "e1" => {
                // Pop register pair H & L off stack
                println!("e1: POP H");
                position += 1;
            }
            "e3" => {
                // Exchange top of stack, H & L.
                // The content of the L register is exchanged with the content of the memory location whose address is specified by the content of register SP.
                // The content of the H register is exchanged with the content of the memory location whose address is one more than the content of register SP.
                println!("e3: XTHL");
                position += 1;
            }
            "e5" => {
                // Push register Pair H & L on stack
                println!("e5: PUSH H");
                position += 1;
            }
            "e6" => {
                // And immediate with A.
                // The content of the second byte of the instruction is logically anded with the contents of the accumulator.
                // The result is placed in the accumulator. The CY and AC flags are cleared.
                println!("e6: ANI D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "e9" => {
                // H & L to program counter.
                // The content of register H is moved to the high-order eight bits of register PC.
                // The content of register l is moved to the low-order eight bits of register PC.
                println!("e9: PCHL");
                position += 1;
            }
            "eb" => {
                // Exchange D & E, H& L Registers
                println!("eb: XCHG");
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
            "f6" => {
                // does an or of A and the byte data, loads that into the accumulator.
                println!("f6: ORI D8. {:x}", buffer[position + 1]);
                position += 2;
            }
            "fa" => {
                //	if M, PC <- adr
                // Jump on Minus
                // ok, cool. you can see this in 4-11
                // minus is if S = 1
                // S is a flag that stands for Sign
                println!(
                    "fa: JM adr. {:x} {:x}",
                    buffer[position + 1],
                    buffer[position + 2]
                );
                position += 3;
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
        if position == end_position {
            println!("Successfully parsed file");
        }
    }
    // for i in buffer.iter() {
    // }
}

