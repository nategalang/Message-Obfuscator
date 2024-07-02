// Huffman Encoding and Encoding: Paolo Estavillo
use std::{collections::{HashMap, BinaryHeap}, cmp::Ordering};

fn get_frequency_arr(text: &Vec<u8>) -> Vec<(u8, i32)> {

    // Build frequency map
    let mut char_map: Vec<i32> = vec![0; 256 + 2];
    for &c in text {
        char_map[c as usize] += 1;
    }

    // Store into a vector
    let mut v: Vec<(u8, i32)> = Vec::new();
    for (i, val) in char_map.iter().enumerate() {
        if char_map[i] == 0 {
            continue;
        }

        v.push((i as u8, *val));
    }

    // Sort by frequency
    v.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    return v;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum HNodeType {
    Parent,
    Leaf,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct HNode {
    node_val: u8,
    node_type: HNodeType,
    freq: i32,
    l_child: Option<Box<HNode>>,
    r_child: Option<Box<HNode>>,
}

impl Ord for HNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.freq != other.freq {
            // self.freq < other.freq
            return other.freq.cmp(&self.freq);
        } else {
            // Equal frequencies
            if self.node_type != other.node_type {
                if self.node_type == HNodeType::Parent {
                    // Prioritize Parent nodes first
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            } else {
                // Equal frequencies and node types
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for HNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn build_htree(frequency_arr: &Vec<(u8, i32)>) -> HNode {

    // Initialize leaf nodes
    let mut pq: BinaryHeap<HNode> = BinaryHeap::new();
    for &(c, val) in frequency_arr {
        pq.push(
            HNode { 
                node_val: c,
                node_type: HNodeType::Leaf, 
                freq: val, 
                l_child: None, 
                r_child: None 
            }
        );
    }

    // Perform huffman loop
    loop {
        if pq.len() < 2 {
            break;
        }

        let u1 = pq.pop().unwrap();
        let u2 = pq.pop().unwrap();
        
        let new_freq = u1.freq + u2.freq;

        let (left_child, right_child) = match u1.cmp(&u2) {
            Ordering::Greater => (u1, u2),
            Ordering::Less => (u2, u1),
            Ordering::Equal => (u1, u2),
        };

        let new_node = HNode {
            node_val: 0_u8,
            node_type: HNodeType::Parent,
            freq: new_freq,
            l_child: Some(Box::new(left_child)),
            r_child: Some(Box::new(right_child)),
        };

        pq.push(new_node);
    }

    return pq.pop().unwrap();

}

fn build_codebook (htree_node: &HNode, codeword_stack: String, codeword_map: &mut HashMap<u8, String>) {
    if (*htree_node).node_type == HNodeType::Leaf {
        // Leaf node reached
        if codeword_stack.is_empty() {
            // Edge case: Only one node in huffman tree
            codeword_map.insert((*htree_node).node_val, String::from("0"));
        } else {
            codeword_map.insert((*htree_node).node_val, codeword_stack);
        }
    } else {
        // Go to left child
        if let Some(left_child) = &(htree_node.l_child) {
            build_codebook(&(**left_child),
                           format!("{}0", &codeword_stack),
                           codeword_map);
        }

        // Go to right child
        if let Some(right_child) = &(htree_node.r_child) {
            build_codebook(&(**right_child), 
                           format!("{}1", &codeword_stack),
                           codeword_map);
        }
    }
}

fn get_encoded_bits(codeword_map: &HashMap<u8, String>, message: &Vec<u8>) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::new();
    for &num in message {
        let s = codeword_map.get(&num).unwrap();
        for c in s.chars() {
            bits.push(
                match c {
                    '1' => 1,
                     _ => 0,
                }
            );
        }
    }

    return bits;
}

fn get_bit_str(n: i128, l: u8) -> String {

    let mut v: String = String::new();
    for i in 0..l {
        let b = if (n & (1 << i)) != 0 {
            '1'
        } else {
            '0'
        };
        v.push(b);
    }

    v = v.chars().rev().collect();

    return v;
}

fn get_canon_freqs(cm: &HashMap<u8, String>) -> Vec<u8> {

    let mut v: Vec<(u8, String)> = Vec::new();
    let mut max_key: u8 = 0;
    for (c, val) in cm {
        v.push((*c, (*val).clone()));
        max_key = max_key.max(*c);
    }

    // We only consider integers from 0 to 9 (0 to 7 for diropql and + 2 for rle)
    let mut ret: Vec<u8> = vec![0; 10];
    if max_key > 9 { // Adjust for ascii values
        ret = vec![0; 256 + 2];
    }

    // Map characters to non-canon codeword length
    for (c, cw) in &v {
        ret[*c as usize] = cw.len() as u8;
    }

    return ret;
}

fn reconstruct_canon_cb(canon_freqs: &Vec<u8>) -> HashMap<u8, String> {
    let mut v: Vec<(u8, u8)> = Vec::new();
    for (i, &val) in canon_freqs.iter().enumerate() {
        if val == 0 { // Only consider at non-zero values
            continue;
        }

        v.push((i as u8, val));
    }

    // Sort by frequencies
    v.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    // Build canonical codebook
    let mut ret: HashMap<u8, String> = HashMap::new();
    let mut l: u8 = 0;
    let mut c_canon: i128 = -1;
    for (c, f) in &v {
        c_canon += 1;
        while *f > l {
            c_canon <<= 1;
            l += 1;
        }

        ret.insert(*c, get_bit_str(c_canon, l));
    }

    return ret;
}

pub fn encode(text: &Vec<u8>) -> (Vec <u8>, Vec<u8>) {

    if text.is_empty() {
        return (vec![], vec![0; 10]);
    }

    let v = get_frequency_arr(&text);
    let ht = build_htree(&v);
    let mut cm: HashMap<u8, String> = HashMap::new();
    build_codebook(&ht, String::from(""), &mut cm);
    let canon_freqs = get_canon_freqs(&cm);
    let canon_cm = reconstruct_canon_cb(&canon_freqs);
    let encoded_bits = get_encoded_bits(&canon_cm, &text);

    return (encoded_bits, canon_freqs);
}

pub fn decode(data: &Vec<u8>, canon_freqs: &Vec<u8>) -> Vec<u8> {

    for bit in data {
        assert!(*bit == 0 || *bit == 1);
    }

    let canon_cb = reconstruct_canon_cb(canon_freqs);
    // Reverse canonized codebook from string to u8
    let mut rev_canon_cb: HashMap<String, u8> = HashMap::new();
    for (key, value) in &canon_cb {
        rev_canon_cb.insert((*value).clone(), *key);
    }

    let mut s = String::new();
    let mut ret: Vec<u8> = Vec::new();
    // For every bit from left to right
    for bit in data {
        // Push the bit in 1
        s.push((bit + '0' as u8) as char);

        // Check if s contains a mapping
        if rev_canon_cb.contains_key(&s) {
            let symb = rev_canon_cb.get(&s).copied().unwrap();
            ret.push(symb);
            s.clear();
        }
    }

    return ret;
}