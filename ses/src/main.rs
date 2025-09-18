fn main() {

    let entries_a = vec!["A", "G", "G", "T", "A", "B"];
    let entries_b = vec!["G", "X", "T", "X", "A", "Y", "B"];

    let mut dp: Vec<Vec<i32>> = vec![vec![0; entries_b.len() + 1]; entries_a.len() + 1];

    for i in 1..=entries_a.len()  {
        dp[i][0] = i as i32;
    }

    for j in 1..=entries_b.len()  {
        dp[0][j] = j as i32;
    }

    for i in 1..=entries_a.len() {
        for j in 1..=entries_b.len() {
            if entries_a[i-1] == entries_b[j-1] {
                dp[i][j] = dp[i-1][j-1]; // match: recebe diagonal
            }
            else {
                dp[i][j] = 1 + dp[i-1][j].min(dp[i-1][j-1]).min(dp[i][j-1]); // missmatch: mais rapido entre deletar, inserir ou substituir
            }
        }
    }

    let mut i = entries_a.len();
    let mut j = entries_b.len();

    let mut ops: Vec<String> = Vec::new();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && entries_a[i-1] == entries_b[j-1] {
            ops.push(format!("Match valor={}", entries_a[i-1]));

            i -= 1;
            j -= 1;
        }
        else if i > 0 && j > 0 && dp[i][j] == dp[i-1][j-1] + 1 {
            ops.push(format!(
                "Substituir pos={} de={} para={}",
                i-1, entries_a[i-1], entries_b[j-1]
            ));
            i -= 1;
            j -= 1;
        }
        else if i > 0 && dp[i][j] == dp[i-1][j] + 1 {
            ops.push(format!("Deletar pos={} valor={}", i-1, entries_a[i-1]));
            i -= 1;
        }
        else if j > 0 && dp[i][j] == dp[i][j-1] + 1 {
            ops.push(format!("Inserir pos={} valor={}", i, entries_b[j-1]));
            j -= 1;
        }
        else if i > 0 {
            ops.push(format!("Deletar pos={} valor={}", i-1, entries_a[i-1]));
            i -= 1;
        }
        else if j > 0 {
            ops.push(format!("Inserir pos={} valor={}", i, entries_b[j-1]));
            j -= 1;
        }
    }

    ops.reverse();

    println!("Quantidade de operações (SES): {}", dp[entries_a.len()][entries_b.len()]);
    println!("Todas as operações: {:?}", ops);
}


fn ses_with_visual_moves() {
    let entries_a = vec!["A", "G", "G", "T", "A", "B"];
    let entries_b = vec!["G", "X", "T", "X", "A", "Y", "B"];

    let mut dp: Vec<Vec<i32>> = vec![vec![0; entries_b.len() + 1]; entries_a.len() + 1];

    // casos base
    for i in 1..=entries_a.len()  {
        dp[i][0] = i as i32;
    }
    for j in 1..=entries_b.len()  {
        dp[0][j] = j as i32;
    }

    // cabeçalho da grid
    print!("    ");
    for b in &entries_b {
        print!("{:>3}", b);
    }
    println!();

    for i in 0..=entries_a.len() {
        if i > 0 {
            print!("{:>3}", entries_a[i-1]);
        } else {
            print!("   ");
        }

        for j in 0..=entries_b.len() {
            print!("{:>3}", dp[i][j]);
        }
        println!();
    }
    println!("============================");

    // preenche a dp com debug
    for i in 1..=entries_a.len() {
        for j in 1..=entries_b.len() {
            if entries_a[i-1] == entries_b[j-1] {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = 1 + dp[i-1][j].min(dp[i-1][j-1]).min(dp[i][j-1]);
            }

            // mostra o estado da matriz a cada passo
            println!("\nApós preencher dp[{}][{}] ({} vs {}):", i, j, entries_a[i-1], entries_b[j-1]);
            print!("    ");
            for b in &entries_b {
                print!("{:>3}", b);
            }
            println!();

            for x in 0..=entries_a.len() {
                if x > 0 {
                    print!("{:>3}", entries_a[x-1]);
                } else {
                    print!("   ");
                }

                for y in 0..=entries_b.len() {
                    print!("{:>3}", dp[x][y]);
                }
                println!();
            }
        }
    }

    println!("\nDistância de edição (SES): {}", dp[entries_a.len()][entries_b.len()]);
}