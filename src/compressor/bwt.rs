#![allow(warnings)]
// lexicographic order for pairs
fn leq_p(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    (a1 < b1) || (a1 == b1) && (a2 <= b2)
}

// lexicographic order for triplets
fn leq_t(a1: i32, a2: i32, a3: i32, b1: i32, b2: i32, b3: i32) -> bool {
    (a1 < b1) || (a1 == b1) && leq_p(a2, a3, b2, b3)
}

fn radixPass(a: &Vec<i32>, b: &mut Vec<i32>, r: &[i32], n: i32, K: i32) {
    // Initialize counter array to 0
    let mut c: Vec<i32> = vec![0; (K + 1) as usize];
    
    // count occurences
    for i in 0..n {
        c[r[a[i as usize] as usize] as usize] += 1;
    }

    // exclusive prefix sums
    let mut sum: i32 = 0;
    for i in 0..=K {
        let t = c[i as usize];
        c[i as usize] = sum;
        sum += t;
    }

    // sort
    for i in 0..n {
        b[c[r[a[i as usize] as usize] as usize] as usize] = a[i as usize];
        c[r[a[i as usize] as usize] as usize] += 1;
    }
}

// find the suffix array SA of T[0..n-1] in {1..K}^n
// require T[n] = T[n + 1] = T[n + 2] = 0, n >= 2
fn construct_suffix_array(T: &Vec<i32>, SA: &mut Vec<i32>, n: i32, K: i32) {
    let n0 = (n + 2) / 3;
    let n1 = (n + 1)/3;
    let n2 = n / 3;
    let n02 = n0 + n2;
    let mut R: Vec<i32> = vec![0; (n02 + 3) as usize];
    let mut SA12: Vec<i32> = vec![0; (n02 + 3) as usize];
    let mut R0: Vec<i32> = vec![0; n0 as usize];
    let mut SA0: Vec<i32> = vec![0; n0 as usize];

    // dbg!(&SA);

    //******* Step 0: Construct sample ********
    // generate positions of mod 1 and mod 2 suffixes
    // the "+(n0-n1)" adds a dummy mod 1 suffix if n%3 == 1
    let mut j: usize = 0;
    for i in 0..(n+(n0 - n1)) {
        if (i % 3) != 0 {
            R[j] = i;
            j += 1;
        }
    }

    //******* Step 1: Sort sample suffixes ********
    // lsb radix sort the mod 1 and mod 2 triples
    radixPass(&R, &mut SA12, &T[2..], n02, K);
    radixPass(&SA12, &mut R, &T[1..], n02, K);
    radixPass(&R, &mut SA12, &T, n02, K);

    // find lexicographic names of triples and
    // write them to correct places in R
    let mut name: i32 = 0;
    let mut c0: i32 = -1;
    let mut c1: i32 = -1;
    let mut c2: i32 = -1;
    for i in 0..(n02 as usize) {
        if T[SA12[i] as usize] != c0 || T[(SA12[i] + 1) as usize] != c1 || T[(SA12[i] + 2) as usize] != c2 {
            name += 1;
            c0 = T[SA12[i] as usize];
            c1 = T[(SA12[i] + 1) as usize];
            c2 = T[(SA12[i] + 2) as usize];
        }
        if (SA12[i] % 3) == 1 {
            R[(SA12[i] / 3) as usize] = name; // write to R1
        } else {
            R[(SA12[i] / 3 + n0) as usize] = name; // write to R2
        }
    }

    // recurse if names are not yet unique
    if name < n02 {
        construct_suffix_array(&mut R, &mut SA12, n02, name);
        // store unique names in R using the suffix array
        for i in 0..(n02 as usize) {
            R[SA12[i] as usize] = (i + 1) as i32;
        }
    } else {
        // generate the suffix array of R directly
        for i in 0..(n02 as usize) {
            SA12[(R[i] - 1) as usize] = i as i32;
        }
    }

    //******* Step 2: Sort nonsample suffixes ********
    // stably sort the mod 0 suffixes from SA12 by their first character
    let mut j: usize = 0;
    for i in 0..(n02 as usize) {
        if SA12[i] < n0 {
            R0[j] = 3*SA12[i];
            j += 1;
        }
    }
    radixPass(&R0, &mut SA0, &T, n0, K);

    //******* Step 3: Merge ********
    // merge sorted SA0 suffixes and sorted SA12 suffixes
    // GetI = if SA12[t] < n0 {SA12[t] * 3 + 1} else { (SA12[t] - n0) * 3 + 2 }
    let mut p: i32 = 0;
    let mut t: i32 = n0-n1;
    let mut k: i32 = 0;
    while k < n {
        let i = if SA12[t as usize] < n0 {SA12[t as usize] * 3 + 1} else { (SA12[t as usize] - n0) * 3 + 2 };
        let j = SA0[p as usize];

        // different compares for mod 1 and mod 2 suffixes
        let comp = if SA12[t as usize] < n0 {
            leq_p(T[i as usize], R[(SA12[t as usize] + n0) as usize], T[j as usize], R[(j/3) as usize])
        } else {
            leq_t(T[i as usize], T[(i + 1) as usize], R[(SA12[t as usize] - n0+1) as usize], T[j as usize], T[(j + 1) as usize], R[(j/3 + n0) as usize])
        };

        if comp {
            // suffix from SA12 is smaller
            SA[k as usize] = i; t += 1;
            if t == n02 { // done --- only SA0 suffixes left
                k += 1;
                while p < n0 {
                    SA[k as usize] = SA0[p as usize];
                    p += 1;
                    k += 1;
                }
            } 
        } else { // suffix from SA0 is smaller
            SA[k as usize] = j; p += 1;
            if p == n0 {
               k += 1;
               while t < n02 {
                SA[k as usize] = if SA12[t as usize] < n0 {SA12[t as usize] * 3 + 1} else { (SA12[t as usize] - n0) * 3 + 2 };
                t += 1;
                k += 1;
               } 
            }
        }

        // Next iteration
        k += 1;
    }
}

pub fn get_sentinel_idx(text: &String) -> usize {
    let mut idx: usize = 0;
    for (i, c) in text.chars().enumerate() {
        if c as u8 == 0 {
            idx = i;
            break;
        }
    }

    return idx;
}

fn radix_sort(t: &mut Vec<[i32; 2]>) { // O(kN)
    let n = t.len();
    let maxi = n.max(300);
    let mut c: Vec<i32> = vec![0; maxi];

    // Sort from rightmost element to leftmost element
    for k in (0..2).rev() {
        // Initialize frequency array
        c.fill(0);

        // Update frequency array
        for i in 0..n {
            c[t[i][k] as usize] += 1;
        }

        // Compute prefix sums
        for i in 1..maxi {
            c[i] += c[i - 1];
        }

        // sort t in place
        let mut temp_t: Vec<[i32; 2]> = vec![[0, 0]; n];
        for i in (0..n).rev() {
            let val = t[i][k] as usize;
            temp_t[(c[val] - 1) as usize] = t[i];
            c[val] -= 1;
        }
        *t = temp_t.clone();
    }
}

pub fn encode(text: &String) -> (String, usize) {
    // Initialize number array
    let mut t: Vec<i32> = text.chars().map(|c| c as i32).collect();
    t.push(0); // Insert sentinel character

    let n = t.len() as i32; // Length of original string + sentinel
    let K: i32 = 256; // ASCII chars as keys

    // Insert 3 zeros at the end
    t.push(0); t.push(0); t.push(0);

    // construct suffix array using DC3
    let mut suffix_array: Vec<i32> = vec![0; n as usize];
    construct_suffix_array(&t, &mut suffix_array, n, K);

    // Get BWT index, i.e., the index of 0 in the suffix array
    let mut idx: usize = 0;
    let n = suffix_array.len() as i32;
    for i in 0..suffix_array.len() {
        if suffix_array[i] == 0 {
            idx = i;
        }

        // decrement by -1 but maintain mod n
        suffix_array[i] = (((suffix_array[i] - 1) % n) + n) % n;
    }

    // Generate BWT string
    let mut v = String::new();
    for i in 0..(n as usize) {
        v.push(t[suffix_array[i] as usize] as u8 as char);
    }

    return (v, idx);
}

pub fn decode(text: &String, index: usize) -> String {
    let mut m = String::new();
    let n = text.len();

    // Create tuples of (char, i)
    let mut v: Vec<[i32; 2]> = Vec::new();
    for (i, c) in text.chars().enumerate() {
        v.push([c as i32, i as i32]);
    }

    // dbg!(&v.len());

    // Radix-sort the tuples
    radix_sort(&mut v);

    // dbg!(&v);

    // Inverse BWT
    let mut curr_index = index;
    // dbg!(n);
    for _ in 0..v.len() {
        m.push((v[curr_index][0] as u8) as char);
        dbg!(v[curr_index][0]);
        curr_index = v[curr_index][1] as usize;
    }

    // Remove sentinel character
    m.pop();

    return m;
}