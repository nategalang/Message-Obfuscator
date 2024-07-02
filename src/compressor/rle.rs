// Run Length Encoding and Decoding: Paolo Estavillo

fn get_msb(n: i128) -> i128 {
    let mut n = n;
    if n == 0 {
        return 0;
    }

    let mut num_bits = 0;
    while n != 0 {
        num_bits += 1;
        n >>= 1;
    }    
    
    return 1 << (num_bits - 1);
}

pub fn encode(text: &Vec<u8>) -> Vec<u8> {
    let mut encoded_sequence: Vec<u8> = Vec::new();

    let mut n_zero: i128 = 0;
    for i in 0..=text.len() {
        if (i <= text.len() - 1) && (text[i] == 0) {
            n_zero += 1;
        } else {
            // Increment n_zero
            n_zero += 1;

            // Push each digit from lsb until msb
            let lim = get_msb(n_zero);
            let mut shamt: i128 = 0;
            while ((1 as i128) << shamt) < lim {
                encoded_sequence.push(
                    if (n_zero & (1 << shamt)) == 0 {0} else {1}
                );
                shamt += 1;
            }

            n_zero = 0; // Reset N_zero to 0
            if i <= text.len() - 1 {
                encoded_sequence.push(text[i] + 2); // push s_i + 2
            }
        }
    }

    return encoded_sequence;
}

fn bits_to_n(bit_stack: &Vec<u8>) -> i32 {
    let mut ret:i32 = 0;
    for (i, bit) in bit_stack.iter().enumerate() {
        ret |= (*bit as i32) << i;
    }
    return ret;
}

pub fn decode(data: &Vec<u8>) -> Vec<u8> {
    let mut decoded_sequence: Vec<u8> = Vec::new();
    let mut n_zero: Vec<u8> = Vec::new();

    for i in 0..=data.len() {
        if (i < data.len()) && (data[i] == 0 || data[i] == 1) {
            // Push a_i into N_zero
            n_zero.push(data[i]);
        } else {
            // Push a 1
            n_zero.push(1);

            // Reverse bits and convert to decimal
            let mut n_zeros_num = bits_to_n(&n_zero);
            n_zeros_num -= 1;
            for _ in 0..n_zeros_num {
                decoded_sequence.push(0);
            }
            n_zero.clear();

            if i < data.len() {
                decoded_sequence.push(data[i] - 2);
            }
        }
    }

    return decoded_sequence;
}