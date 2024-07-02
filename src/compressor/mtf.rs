// Move to Front Encoding and Decoding: Paolo Estavillo

fn search(alphabet: &String, c: char) -> usize {
    assert!(alphabet.contains(c), "character not found in alphabet");

    alphabet.find(c).unwrap()
}

pub fn encode(text: &String, alphabet: &String) -> Vec<u8> {
    let mut s: Vec<u8> = Vec::new();
    let mut current_alphabet = alphabet.clone();
    
    for c in text.chars() {
        // Get position
        let pos = search(&current_alphabet, c);

        // Push to s
        s.push(pos as u8);

        // move to front
        current_alphabet.remove(pos);
        current_alphabet.insert(0, c);
    }

    return s;
}

pub fn decode(data: &Vec<u8>, alphabet: &String) -> String {
    let mut s = String::new();
    let current_alphabet = alphabet.clone();
    let mut current_alphabet: Vec<char> = current_alphabet.chars().collect();

    for &pos in data {
        // Get character at pos
        let c = current_alphabet[pos as usize];

        // Push to reconstructed message
        s.push(c);

        // Move to front
        current_alphabet.remove(pos as usize);
        current_alphabet.insert(0, c);
    }

    return s;
}