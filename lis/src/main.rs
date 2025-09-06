fn main() {
    println!("Resolvendo o problema da LIS!");

    let entries = vec![10, 22, 9, 33, 21, 50, 41, 60];
    let mut tails: Vec<i32> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();
    let mut prev_index: Vec<Option<usize>> = vec![None; entries.len()];

    for (index, entry) in entries.iter().enumerate() {
        let pos = match tails.binary_search(entry) {
            Ok(pos) => pos,
            Err(pos) => pos
        };

        if pos == tails.len() {    
            tails.push(*entry);
            indices.push(index);
        }
        else {
            tails[pos] = *entry;
            indices[pos] = index;
        }

        if pos == 0 {
            prev_index[index] = None;
        }
        else {
            prev_index[index] = Some(indices[pos - 1]);
        }
    }

    let mut last_index: Option<usize> = Some(indices[tails.len() - 1]);
    let mut lis: Vec<i32> = Vec::new();

    while last_index != None {
        let i = last_index.unwrap();
        lis.push(entries[i]);

        last_index = prev_index[i];
    }

    lis.reverse();

    println!("Tails:");
    for tail in tails {
        print!("{}", tail);
    }
    
    println!("\nIndices:");
    for index in indices {
        print!("{}", index);
    }

    println!("\nLis:");
    for index in lis {
        print!("{}", index);
    }
}
