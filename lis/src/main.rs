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
    for idx in &indices {
        print!("{}", idx);
    }

    println!("\nLis:");
    for index in lis {
        print!("{}", index);
    }
}

//Explicação do algoritimo:
// 1. Para cada elemento da sequencia, realizar uma busca binária para saber onde ele se encaixa em TAILS
//      1.1 Se a busca binária retornar na ultima posição: realizar um tails.push(entry)
//      1.2 Se a busca binária retornar alguma posição especifica: editar o valor tails[pos] = entry
// 2 Para cada elemento adicionado em tails, armazenar a posição do elemento na sequencia original em uma lista chamada INDICES
// 3. Para cada elemento da sequencia, adicionar o elemento anterior na lista PREV_INDEX
//      3.1 Se o elemento percorrido, é o primeiro da sequencia: PREV_INDEX[0] = NONE
//      3.2 Se o elemento percorrido, for menor que o primeiro elemento de tails: PREV_INDEX[INDEX] = NONE
//      3.3 Ó objetivo dessa lsita é armazenar o valor anterior de TAILS, será utilizado para montar a LIS.
// 4. Após montar TAILS, INDICES e PREV_INDEX, a lista de LIS será montada.
//      4.1 Começar um while com o last_index sendo o ultimo elemento de indices.
//      4.2 last_index será armazenado em lis: lis.push(last_index)
//      4.3 