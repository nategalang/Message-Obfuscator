/* write function increments or decrements the content of memory cell index 0 until it is equal
to the ASCII code of the character in the text; then push it to the output queue */

pub fn write(text: &String) -> String {
    let mut diropql_program = String::new();    // diropql program code
    let mut mp_0: u8 = 0;   // Content of memory cell index 0

    for c in text.chars() {
        let ascii_code = c as u8;   // ASCII code of the character

        while mp_0 != ascii_code {  // Loop until content of memory cell index 0 is equal to ASCII code of the character

            if mp_0 > ascii_code {

                if mp_0 - ascii_code > 128 {    // More efficient to add if this is the case, e.g. 255 -> 4 (5 moves)
                        
                    if mp_0 < 255 {
                        mp_0 = mp_0 + 1;
                    }

                    else {
                        mp_0 = 0;   // Wrap around
                    }

                    diropql_program.push('i');  // i to diropql program code
                }

                else {
                    mp_0 = mp_0 - 1;    // More efficient to subtract if this is the case, e.g. 255 -> 250 (5 moves)
                    diropql_program.push('d');  // d to diropql program code
                }
            }

            else {

                 if ascii_code - mp_0 > 128 {    // More efficient to subtract if this is the case, e.g. 4 -> 255 (5 moves)
                        
                    if mp_0 > 0 {
                        mp_0 = mp_0 - 1;
                    }

                    else {
                        mp_0 = 255; // Wrap around
                    }

                    diropql_program.push('d');  // d to diropql program code
                }

                else {
                    mp_0 = mp_0 + 1;    // More efficient to add if this is the case, e.g. 250 -> 255 (5 moves)
                    diropql_program.push('i');  // i to diropql program code
                }
            }
        }

        // o to diropql program code once content of memory cell index 0 equals ASCII code of the character
        diropql_program.push('o');  
    }

    return diropql_program;
}

// read function reads a diropql program code and returns the text

pub fn read(prog: &String) -> String {
    let mut memory_cells: Vec<u8> = vec![0; 10000]; // Initialize 10,000 memory cells to 0

    // Initialize memory pointer, instruction pointer, and output queue
    let mut mp: usize = 0;
    let mut ip: usize = 0;
    let mut oq: Vec<u8> = Vec::new();

    // Record indices of matching p and q commands in a vector
    let mut pq_index: Vec<(usize, usize)> = Vec::new();

    // Algorithm to pair the indices of the p and q commands
    let mut p_indices: Vec<usize> = Vec::new();

    for (index, c) in prog.chars().enumerate() {

        if c == 'p' {
            p_indices.push(index);
        } 
        
        else if c == 'q' {

            if let Some(p_index) = p_indices.pop() {
                pq_index.push((p_index, index));
            }
        }
    }

    // Read each command in the diropql program code
    while ip != prog.len() {
        let c = prog.chars().nth(ip);

        if c == Some('l') { // Decrement mp

            if mp == 0 {    // Wrap around
                mp = 9999;
            }

            else {
                mp = mp - 1;
            }
        }

        else if c == Some('r') {    // Increment mp

            if mp == 9999 { // Wrap around
                mp = 0;
            }

            else {
                mp = mp + 1;
            }
        }

        else if c == Some('i') {    // Increment content pointed by mp

            if memory_cells[mp] == 255 {    // Wrap around
                memory_cells[mp] = 0;
            }

            else {
                memory_cells[mp] = memory_cells[mp] + 1;
            }
        }

        else if c == Some('d') {    // Decrement content pointed by mp

            if memory_cells[mp] == 0 {  // Wrap around
                memory_cells[mp] = 255;
            }

            else {
                memory_cells[mp] = memory_cells[mp] - 1;
            }
        }

        else if c == Some('o') {    // Push content pointed by mp to output queue
            oq.push(memory_cells[mp]);
        }

        else if c == Some('p') {  // Change ip to the index of the matching q command

            if memory_cells[mp] == 0 {

                for pq in &pq_index {

                    if pq.0 == ip {
                        ip = pq.1;
                    }
                }
            }
        }

        else if c == Some('q') {    // Change ip to the index of the maching p command

            if memory_cells[mp] != 0 {
                    
                for pq in &pq_index {

                    if pq.1 == ip {
                        ip = pq.0;
                    }
                }
            }
        }

        ip = ip + 1;    // Increment instruction pointer after each command
    }

    // Convert ASCII codes in the output queue to string
    let mut output_string = String::new();

    for ascii_code in oq {
        let ascii_char = std::char::from_u32(ascii_code as u32).unwrap();
        output_string.push(ascii_char);
    }

    return output_string;
}

// Submodule zip
pub mod zip;

#[cfg(test)]
mod dpql_test {
	use super::*;
    #[test]
	fn write_pt1_empty() {
		// Empty string test
		// Passes empty string into write(), should return empty
		
		let string = String::from("");
		let expected = String::from("");
		let received = write(&string);
		
		assert_eq!(expected, received);		
	}
	
	#[test]
	fn read_pt1_empty() {
		// Empty string test
		// Passes empty string into read (), should return empty
		
		let string = String::from("");
		let expected = String::from("");
		let received = read(&string);
		
		assert_eq!(expected, received);		
	}

	#[test]
	fn read_pt2_non_dpql() {
		// Test that checks if the interpreter ignores non dpql commands.
		// Should return an empty string
		
		let string = String::from("abcwxyz");
		let expected = String::from("");
		let received = read(&string);
		
		assert_eq!(expected, received);
	}
	
	#[test]
	fn read_pt3() {
		
		// Increment the first memory cell 65 times
		let mut string = String::from("");
		for _i in 0..65 {
			string.push('i');
		}		
		
		// Then output the contents to the oq.
		string.push('o');
		
		// oq will have one item, 65 which converts to "A" in ASCII
		let expected = String::from("A");
		let received = read(&string);
		
		assert_eq!(expected, received);
	}
	
	#[test]
	fn read_pt4_wrap() {
		// Tests the wrapping of the memory pointer around the cells
		
		// Increment the first memory cell 65 times
		let mut string = String::from("");
		for _i in 0..65 {
			string.push('i');
		}
		
		// Increment the memory pointer until it wraps around to the first cell
		for _j in 0..10000 {
			string.push('r');
		}
		
		// Output contents of the current cell to the oq
		string.push('o');
		
		// oq will have one item, 65, which converts to "A" in ASCII
		let expected = String::from("A");
		let received = read(&string);
		
		assert_eq!(expected, received);
	}
	
	#[test]
	fn read_pt5_overflow() {
		// Tests the overflow wrapping of the memory cells
		
		// Increments the first memory cell 256 times (wraps around to 0)
		let mut string = String::from("");
		for _i in 0..256 {
			string.push('i');
		}
	
		// Increment contents of the cell 65 times (capital "A")
		for _k in 0..65 {
			string.push('i');
		}
		
		// Push contents of cell into output queue
		string.push('o');

		// oq will have one item, 65 which converts to "A" in ASCII
		let expected = String::from("A");
		let received = read(&string);
		
		assert_eq!(expected, received);	
	}
	
	#[test]
	fn read_pt6_loops() {
		// This part tests the functionality of loops
		
		let mut string = String::from("");
		
		// Increment contents of the cell 65 times
		for _k in 0..65 {
			string.push('i');
		}
		
		// loop through the second memory cell, 65 times
		string.push_str("prildq");
		
		// output contents of second memory cell into queue
		string.push_str("ro");
		
		let expected = String::from("A");
		let received = read(&string);

		assert_eq!(expected, received);
	}
	
	// Write Read tests
	
	#[test]
	fn write_read_pt1() {
		// This tests both the write and read fn for every lower and upper case letter
		
		let string = String::from("the quick brown fox jumps over the lazy dog. THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.");
		let encoded = write(&string);
		let decoded = read(&encoded);
		
		assert_eq!(string, decoded);		// Check if decoded equals original.
	}
	
	#[test]
	fn write_read_pt2() {
		// It tests for every printable ASCII character (characters 32-126)
		
		let string = String::from(" !'\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"); 
		let encoded = write(&string);
		let decoded = read(&encoded);
		
		assert_eq!(string, decoded);		// Check if decoded equals original			
	}
	
	#[test]
	fn write_read_pt3() {
		// Tests for nonprintable whitespaces in ASCII
		// e.g. Tab, Newline, Cariage Return, and Space
	
		let string = String::from("Carriage Return:\rSpace: Newline:\nTab:\t");
		let encoded = write(&string);
		let decoded = read(&encoded);
		
		assert_eq!(string, decoded);
		print!("{}", string);
	}
}