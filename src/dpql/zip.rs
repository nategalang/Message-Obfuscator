#[allow(unused_imports)]
use crate::dpql::*;

#[allow(unused_imports)]
use crate::compressor::*;

use base85::{decode, encode};

// write function takes in a message and converts it to a diropqlz file

pub fn write(text: &String) -> String {
    // Convert the message to a diropql file
    let dpql = crate::dpql::write(text);

    // Compress diropql file using bwt, mtf, rle, and huffman encode functions
    let (bwt_encoded, bwt_index) = crate::compressor::bwt::encode(&dpql);
	let bwt_index_u64: u64 = bwt_index.try_into().unwrap();

    let mtf_encoded = crate::compressor::mtf::encode(&bwt_encoded, &String::from("\0dilopqr"));

    let rle_encoded = crate::compressor::rle::encode(&mtf_encoded);

    let (huffman_encoded, huffman_bitlens) = crate::compressor::huffman::encode(&rle_encoded);

    // Determine the length of the message and the offset
	#[allow(unused_assignments)]
	let mut msg_len: u64 = 0;

	#[allow(unused_assignments)]
	let mut msg_offset: u8 = 0;

    if huffman_encoded.len() % 8 == 0 {
        msg_len = huffman_encoded.len().try_into().unwrap();
        msg_offset = 0;
    }

    else {
        msg_len = (huffman_encoded.len() / 8 + 1).try_into().unwrap();
        msg_offset = (8 - (huffman_encoded.len() % 8)).try_into().unwrap();
    }

    // Define the metadata struct
    let meta_data = DpqlzMeta {
        mlen: msg_len,
        moffset: msg_offset,
        bwt_idx: bwt_index_u64,
        huff_bitlens: huffman_bitlens,
    };

    // Use the write_meta function to convert the compressed diropql file to a diropqlz file
    let dpqlz = write_meta(&meta_data, &huffman_encoded);

    return dpqlz;
}

// read function takes in a diropqlz file and returns the original message

pub fn read(prog: &String) -> String {
    // Use the read_meta function to convert the diropqlz file to the compressed diropql file
    let (meta_data, huffman_encoded) = read_meta(prog);

    // Decompress the diropql file using huffman, rle, mtf, and bwt decode functions
    let huffman_decoded = crate::compressor::huffman::decode(&huffman_encoded, &meta_data.huff_bitlens);

    let rle_decoded = crate::compressor::rle::decode(&huffman_decoded);

    let mtf_decoded = crate::compressor::mtf::decode(&rle_decoded, &String::from("\0dilopqr"));

	let bwt_idx_usize = meta_data.bwt_idx.try_into().unwrap();
    let dpql = crate::compressor::bwt::decode(&mtf_decoded, bwt_idx_usize);

    // Convert the diropql program to the original message
    let text = crate::dpql::read(&dpql);

    return text;
}

// write_meta function converts a compressed diropql program to a diropqlz program

pub fn write_meta(meta: &DpqlzMeta, prog: &Vec<u8>) -> String {

    // Convert obfuscated message's length from u64 to a vector of eight u8 elements
    let msg_len_u64 = meta.mlen;
    let mut msg_len_u8: Vec<u8> = msg_len_u64.to_be_bytes().to_vec();

    // Append obfuscated message's offset to the obfuscated message's length vector
    let msg_offset = meta.moffset;
    msg_len_u8.push(msg_offset);

    // Convert obfuscated message's bwt index from u64 to a vector of eight u8 elements
    let msg_bwt_idx_u64 = meta.bwt_idx;
    let mut msg_bwt_idx_u8: Vec<u8> = msg_bwt_idx_u64.to_be_bytes().to_vec();

    // Insert 6 zeros at the end of the obfuscated message's canonical Huffman codebook
    let mut msg_huff_bitlens = meta.huff_bitlens.clone();

    for _n in 0..6 {
        msg_huff_bitlens.push(0);
    }

    // Append the offset zeros at the end of the obfuscated message
    let mut prog_b = prog.clone();

    for _n in 0..msg_offset {
        prog_b.push(0);
    }

    // Convert the obfuscated message from binary to a vector of u8 elements
    let mut b_str = String::new();
    let mut prog_u8: Vec<u8> = Vec::new();

    for bit in &prog_b {

        if b_str.len() != 8 {
            b_str.push_str(&bit.to_string());
        }

        else {
            prog_u8.push(u8::from_str_radix(&b_str, 2).unwrap());
            b_str = String::from(&bit.to_string());
        }
    }

    if prog_b.len() != 0 {
        prog_u8.push(u8::from_str_radix(&b_str, 2).unwrap());
    }

    // Combine all the vector of u8 elements into a single vector
    let mut msg: Vec<u8> = Vec::new();
    msg.extend(msg_len_u8);
    msg.extend(msg_bwt_idx_u8);
    msg.extend(msg_huff_bitlens);
    msg.extend(prog_u8);
    
    // Encode the message using the Base85 encode function
    let mut encoded_msg = encode(&msg);
    encoded_msg.insert_str(0, &String::from("DIROPQLZ"));   // Prepend the magic string

    return encoded_msg;
}

// read_meta function converts a diropqlz program to a compressed diropql program

pub fn read_meta(prog: &String) -> (DpqlzMeta, Vec<u8>) {
    // Remove the unnecessary characters and the magic string prepended from the diropqlz program
    let mut dpqlz = prog.clone();
    let mut ignore = String::new();

    for c in dpqlz.chars() {

        if ignore.contains(&String::from("DIROPQLZ")) {
            dpqlz = dpqlz.replace(&ignore, &String::from(""));
            break;
        }

        ignore.push(c);
    }

    // Decodes the diropqlz program which returns the vector of bytes containing the metadata and the obfuscated message
    let decoded_msg = decode(&dpqlz).unwrap();

    // Split the vector into the different struct fields and the obfuscated message
    let mut msg_len_u8: Vec<u8> = Vec::new();
    let mut msg_offset: u8 = 0;
    let mut msg_bwt_idx_u8: Vec<u8> = Vec::new();
    let mut msg_huff_bitlens: Vec<u8> = Vec::new();
    let mut prog_u8: Vec<u8> = Vec::new();
    
    for i in 0..decoded_msg.len() {

        if i <= 7 {
            msg_len_u8.push(decoded_msg[i]);
        }

        else if i == 8 {
            msg_offset = decoded_msg[i];
        }

        else if i > 8 && i <= 16 {
            msg_bwt_idx_u8.push(decoded_msg[i]);
        }

        else if i > 16 && i <= 26 {
            msg_huff_bitlens.push(decoded_msg[i]);
        }

        else if i > 32 {
            prog_u8.push(decoded_msg[i]);
        }
    }

    // Convert the vector of bytes of the mlen field to u64
    let mut msg_len_u64: u64 = 0;

    for &byte in &msg_len_u8 {
        msg_len_u64 = (msg_len_u64 << 8) | u64::from(byte);
    }

    // Convert the vector of bytes of the bwt_idx field to u64
    let mut msg_bwt_idx_u64: u64 = 0;

    for &byte in &msg_bwt_idx_u8 {
        msg_bwt_idx_u64 = (msg_bwt_idx_u64 << 8) | u64::from(byte);
    }

    // The fields of the metadata is decoded
    let meta_data = DpqlzMeta {
        mlen: msg_len_u64,
        moffset: msg_offset,
        bwt_idx: msg_bwt_idx_u64,
        huff_bitlens: msg_huff_bitlens,
    };

    // Convert the vector of bytes of the obfuscated message to a single binary string
    let mut prog_b = String::new();
    
    for &byte in &prog_u8 {
        let b_str = format!("{:08b}", byte);
        prog_b.push_str(&b_str);
    }

    // Append each bit in the binary string to a vector of bits
    let mut cmpr_dpql: Vec<u8> = Vec::new();

    for c in prog_b.chars() {
        
        if c == '1' {
            cmpr_dpql.push(1);
        }

        else {
            cmpr_dpql.push(0);
        }
    }

    // Remove the offset bits and the compressed diropql message is decoded
    for _n in 0..msg_offset {
        cmpr_dpql.pop();
    }

    return (meta_data, cmpr_dpql);
}

// Metadata struct used for encoding and decoding of diropqlz program

pub struct DpqlzMeta {
    pub mlen: u64,
    pub moffset: u8,
    pub bwt_idx: u64,
    pub huff_bitlens: Vec<u8>,
}

#[cfg(test)]
mod zip_tests {
    use super::*;

    #[test]
    fn write_pt1_empty() {
	/* Inputs an empty string ""
		
	Outputs at each step:
		dpql:		""	
		bwt:		"\0"
			index:	0
		mtf:		[0]
		rle:		[0]
		huffman:		[0]
			codebook:	[1,0,0,0,0,0,0,0,0,0]
		write_meta():	   
	*/
	
		let message = String::from("");
		let encoded = write(&message);	   
		let expected = String::from("DIROPQLZ00000000012LJ#70000000961000000000000000000");
	   
		assert_eq!(expected, encoded);
    }

    #[test]
    fn read_pt1_empty() {
        // Takes in a dpqlz file; should ouptut empty string
		// write_pt_empty in reverse
		
		let encoded = String::from("DIROPQLZ00000000012LJ#70000000961000000000000000000");		
		let message = read(&encoded);
		let expected = String::from("");
		
		assert_eq!(expected, message);
    }
	
	#[test]
	fn write_pt2_hello() {
		// Hellow world! example ni sir
		// since dpql can be implemented in different, the answer may not be the same
		// however, if read_pt2_hello()and write_read_pt5_hello() work, we should be fine		
		
		let message = String::from("Hello world!");
		let encoded = write(&message);
		let expected = String::from("DIROPQLZ000000000P1ONa400003F9HGp0s{sF1^@*B000000Pk>8&=d<vN`g=*1u04>8~So+F}|CziXZ");
		
		assert_eq!(expected, encoded);
	}
	
	#[test]
	fn read_pt2_hello() {
		// The Hello World test example ni sir
		// I tried reverse engineering it and it seems legit
		// Theoretically, this should work even if the specific dpql implementations aren't the same
		
		let encoded = String::from("DIROPQLZ000000000P1ONa400003F9HGp0s{sF1^@*B000000Pk>8&=d<vN`g=*1u04>8~So+F}|CziXZ");
		let message = read(&encoded);
		let expected = String::from("Hello world!");

		assert_eq!(expected, message);
	}
	
	#[test]
	fn read_pt3_ignore() {
		// This is the same as read_pt2_hello() but with trash characters
		// It should ignore the characters before the magic string DIROPQLZ
		
		let encoded = String::from("qwerty asdfDIROPQLZ000000000P1ONa400003F9HGp0s{sF1^@*B000000Pk>8&=d<vN`g=*1u04>8~So+F}|CziXZ");
		let message = read(&encoded);
		let expected = String::from("Hello world!");

		assert_eq!(expected, message);
	}
	
	#[test]
	fn write_read_pt1_empty() {
		// Inputs an empty string; passes it through the obfuscator then deobfuscator
		// Should return back the empty string
		
		let original = String::from("");
		let encoded = write(&original);
		let decoded = read(&encoded);
		
		assert_eq!(original, decoded);
	}
	
	#[test]
	fn write_read_pt2() {
		// Tests all letters of alphabet, capital and lowercase;
		
		let original = String::from("the quick brown fox jumps over the lazy dog. THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG");
		let encoded = write(&original);
		let decoded = read(&encoded);
		
		assert_eq!(original, decoded);
	}
	
	#[test]
	fn write_read_pt3() {
		// Tests for every printable ASCII character (32-126)
		
		let original = String::from(" !'\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"); 
		let encoded = write(&original);
		let decoded = read(&encoded);
		
		assert_eq!(original, decoded);
	}
	
	#[test]
	fn write_read_pt4() {
		// Tests for nonprintable whitespaces in ASCII
		
		let original = String::from("Carriage Return:\rSpace: Newline:\nTab:\t");
		let encoded = write(&original);
		let decoded = read(&encoded);
		
		assert_eq!(original, decoded);
	}
	
	#[test]
	fn write_read_pt5_hello() {
		// Message is "Hello World!"
		
		let original = String::from("Hello world!");
		let encoded = write(&original);
		let decoded = read(&encoded);
		
		assert_eq!(original, decoded);
	}
	
	#[test]
	fn meta_test() {
		// Simple sanity test for the functionality of the DpqlzMeta struct
		
		let new_meta = DpqlzMeta{
			mlen: 5,
			moffset: 1,
			bwt_idx: 7,
			huff_bitlens: vec![1,0,0,0,0,0,0,0,0,0],
		};
		
		assert_eq!(5, new_meta.mlen);
		assert_eq!(1, new_meta.moffset);
		assert_eq!(7, new_meta.bwt_idx);
		assert_eq!(vec![1,0,0,0,0,0,0,0,0,0], new_meta.huff_bitlens);
	}


	#[test]
	fn write_meta_pt1_empty() {
		// Empty test: inputs an empty program; 
		
		let meta = DpqlzMeta {
			mlen: 0,
			moffset: 0,
			bwt_idx: 0,
			huff_bitlens: vec![0,0,0,0,0,0,0,0,0,0],
		};
		
		let prog: Vec<u8> = vec![];		
		let encoded = write_meta(&meta, &prog);
		
		/* Notes on the expected return string
		
		There are a total of 33 bytes (264 bits) for the prepended metadata
		In this test case, all of them are 0. The message itself is an empty string. 
		Additionally, no bits are appended since total output is already divisible by 8.
		
		Thus, we have 264 bits of 0. This long string of zeros are then encoded to base 85.
		The magic string "DIROPQLZ" is then prepended and this is now the final output.		
		*/
		
		let expected = String::from("DIROPQLZ000000000000000000000000000000000000000000");
		
		assert_eq!(expected, encoded)
	}
	
	#[test]
	fn read_meta_pt1_empty() {
		// Empty string test; does write_meta_pt1 in reverse
		
		// Create program to input into read_meta
		let prog = String::from("DIROPQLZ000000000000000000000000000000000000000000");
		
		// Create expected meta data
		let expected_meta = DpqlzMeta {
			mlen: 0,
			moffset: 0,
			bwt_idx: 0,
			huff_bitlens: vec![0,0,0,0,0,0,0,0,0,0],
		};
		
		let expected: Vec<u8> = vec![];	// The expected decoded is an empty vector
				
		let (meta, decoded) = read_meta(&prog); // Input program into read_meta
		
		
		assert_eq!(expected, decoded);		// Assert correctness of decoded message
		
		// Assert correctness of generated meta data
		assert_eq!(expected_meta.mlen, meta.mlen);
		assert_eq!(expected_meta.moffset, meta.moffset);
		assert_eq!(expected_meta.bwt_idx, meta.bwt_idx);
		assert_eq!(expected_meta.huff_bitlens, meta.huff_bitlens);		
	}
		
	#[test]
	fn write_meta_pt2_unit() {
		// Unit test: inputs a vector with a single element of 0; 
		
		let prog: Vec<u8> = vec![0];		// Single element of 0
		
		let meta = DpqlzMeta {
			mlen: 1,						// Length in bytes of message
			moffset: 7,						// Ignore 7 bits at the end of message
			bwt_idx: 1,									// Index is 0
			huff_bitlens: vec![1,0,0,0,0,0,0,0,0,0],	// huffman codebook
		};		
		
		let encoded = write_meta(&meta, &prog);
		
		/* Notes on the expected return string
		
		There are a total of 33 bytes (264 bits) for the prepended metadata.
		Not all metadata bits are zero. The message itself is 1 bit of 0.
		7 bits will need to be appended to the end of the mesesage to make it 8-divisible.
		
		Thus, we have 272 bits total (34 bytes). This long string is encoded to base 85
		and a magic string "DIROPQLZ" is prepended and this is now the final output.
		*/
		
		let expected = String::from("DIROPQLZ00000000012LJ#7000000RaF2000000000000000000");		

		assert_eq!(expected, encoded)
	}
	
	#[test]
	fn read_meta_pt2_unit() {
		// Unit test:: should output a vector with a single element 0
		// Does the reverse of write_meta_pt2_unit
		
		// Create program to input into read_meta
		let prog = String::from("DIROPQLZ00000000012LJ#7000000RaF2000000000000000000");
		
		// Create expected meta struct
		let expected_meta = DpqlzMeta {
			mlen: 1,						
			moffset: 7,						
			bwt_idx: 1,								
			huff_bitlens: vec![1,0,0,0,0,0,0,0,0,0],	
		};
		
		let (meta, decoded) = read_meta(&prog); 	// Input program into read_meta		
		let expected: Vec<u8> = vec![0];				// The expected decoded is a vector with 0
			
		assert_eq!(expected, decoded);					// Assert correctness of decoded message
		
		// Assert correctness of generated meta data
		assert_eq!(expected_meta.mlen, meta.mlen);
		assert_eq!(expected_meta.moffset, meta.moffset);
		assert_eq!(expected_meta.bwt_idx, meta.bwt_idx);
		assert_eq!(expected_meta.huff_bitlens, meta.huff_bitlens);		
	}
	
	#[test]
	fn write_meta_pt3() {
		// Arbritrary test: input is 54 binary bits long (append 2 zeros to end)
		
		let prog: Vec<u8> = vec![1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,1,1,0,1,1,1,0,1,1,0,1,0,0];
		
		let meta = DpqlzMeta {
			mlen: 7,
			moffset: 2,
			bwt_idx: 9,
			huff_bitlens: vec![1,1,0,1,1,1,1,1,1,1]
		};
		
		let encoded = write_meta(&meta, &prog);
		
		let expected = String::from("DIROPQLZ00000000070ssI2000002>}5B0RaI40RaI3000000RMmgzkTk|");
									
		assert_eq!(expected, encoded);
	}
	
	#[test]
	fn read_meta_pt3() {
		// Arbritrary test; reverse of write_meta_pt3
		
		let prog = String::from("DIROPQLZ00000000070ssI2000002>}5B0RaI40RaI3000000RMmgzkTk|");	

		let expected_meta = DpqlzMeta {
			mlen: 7,
			moffset: 2,
			bwt_idx: 9,
			huff_bitlens: vec![1,1,0,1,1,1,1,1,1,1],
		};
		
		let expected: Vec<u8> = vec![1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,1,1,0,1,1,1,0,1,1,0,1,0,0];
		
		let (meta, decoded) = read_meta(&prog); 			
		
		assert_eq!(expected, decoded);	
		
		assert_eq!(expected_meta.mlen, meta.mlen);
		assert_eq!(expected_meta.moffset, meta.moffset);
		assert_eq!(expected_meta.bwt_idx, meta.bwt_idx);
		assert_eq!(expected_meta.huff_bitlens, meta.huff_bitlens);
	}
	
	#[test]
	fn write_meta_pt4() {
		// Arbritrary test: input is 16 bits long: 1111111100000000
		// No need to put extra bits
		
		let prog: Vec<u8> = vec![1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0];
		
		let meta = DpqlzMeta {
			mlen: 2,
			moffset: 0,
			bwt_idx: 300,
			huff_bitlens: vec![0,0,0,8,8,0,0,0,0,0],
		};
		
		let encoded = write_meta(&meta, &prog);
		
		let expected = String::from("DIROPQLZ00000000020000000001EC2ui2nYZG00000000000RI3");
		
		assert_eq!(expected, encoded);
	}
	
	#[test]
	fn read_meta_pt4() {
		// Arbritrary test: write_meta_pt4 in reverse
		
		let prog = String::from("DIROPQLZ00000000020000000001EC2ui2nYZG00000000000RI3");
		
		let expected_meta = DpqlzMeta {
			mlen: 2,
			moffset: 0,
			bwt_idx: 300,
			huff_bitlens: vec![0,0,0,8,8,0,0,0,0,0],
		};
		
		let expected: Vec<u8> = vec![1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0]; 
		
		let (meta, decoded) = read_meta(&prog); 			
		
		assert_eq!(expected, decoded);	
		
		assert_eq!(expected_meta.mlen, meta.mlen);
		assert_eq!(expected_meta.moffset, meta.moffset);
		assert_eq!(expected_meta.bwt_idx, meta.bwt_idx);
		assert_eq!(expected_meta.huff_bitlens, meta.huff_bitlens);
	}
	
	#[test]
	fn read_meta_pt5_ignore() {
		// This is a rehash of the empty test read_meta_pt1_empty
		// However there are some trash characters before the official string
		// The function must ignore this and only take in starting with DIROPQLZ
		
		let prog = String::from("thequickbrownfoxjumpsoverthelazydogDIROPQLZ000000000000000000000000000000000000000000");
		
		let expected_meta = DpqlzMeta {
			mlen: 0,
			moffset: 0,
			bwt_idx: 0,
			huff_bitlens: vec![0,0,0,0,0,0,0,0,0,0],
		};
		
		let expected: Vec<u8> = vec![];					
		let (meta, decoded) = read_meta(&prog); 		
		
		assert_eq!(expected, decoded);		
		
		assert_eq!(expected_meta.mlen, meta.mlen);
		assert_eq!(expected_meta.moffset, meta.moffset);
		assert_eq!(expected_meta.bwt_idx, meta.bwt_idx);
		assert_eq!(expected_meta.huff_bitlens, meta.huff_bitlens);
	}
} 