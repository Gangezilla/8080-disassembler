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
    // let mut position = 0;
    // let end_position = buffer.len();
    // while position < end_position {

    // }
    for i in buffer.iter() {
        let hex = &format!("{:x}", i);
        match &hex as &str {
            "0" => println!("NOP"),
            _ => println!("Unimplemented instruction"),
        }
    }
}

