fn main() {
    println!("Resolvendo o problema da LCS!");
    let entries_a = vec!["A", "G", "G", "T", "A", "B"];
    let entries_b = vec!["G", "X", "T", "X", "A", "Y", "B"]; 

    let mut dp: Vec<Vec<i32>> = vec![vec![0; entries_b.len() + 1]; entries_a.len() + 1];
    let lcs_len = dp[entries_a.len()][entries_b.len()];

    for i in 1..=entries_a.len() {
        for j in 1..=entries_b.len()  {
            if entries_a[i-1] == entries_b[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
            }
            else {
                dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
            }
        }
    }

    println!("o tamanho da LCS é: {}", lcs_len);

    let mut i = entries_a.len();
    let mut j = entries_b.len();
    let mut lcs: Vec<&str> = Vec::new();

    while i > 0 && j > 0 {
        if entries_a[i-1] == entries_b[j-1] {
            lcs.push(entries_a[i-1]);
            j -= 1;
            i -= 1;
        }
        else if dp[i-1][j] > dp[i][j-1] {
            i -= 1;
        }
        else {
            j -= 1;
        }
    }

    lcs.reverse();
    println!("LCS completa é: {:?}", lcs);
}

// Monta exibição visual da movimentação na etapa de backtracking.
fn lcs_with_visual_moves() {
    println!("Resolvendo o problema da LCS!");
    let entries_a = vec!["A", "G", "G", "T", "A", "B"];
    let entries_b = vec!["G", "X", "T", "X", "A", "Y", "B"]; 

    let mut dp: Vec<Vec<i32>> = vec![vec![0; entries_b.len() + 1]; entries_a.len() + 1];
    let mut lcs_len = 0;

    for i in 1..=entries_a.len() {
        for j in 1..=entries_b.len()  {
            if entries_a[i-1] == entries_b[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
            }
            else {
                dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
            }
        }
    }

    lcs_len = dp[entries_a.len()][entries_b.len()];
    println!("o tamanho da LCS é: {}", lcs_len);

    let mut i = entries_a.len();
    let mut j = entries_b.len();
    let mut lcs: Vec<&str> = Vec::new();

    // Visualização do backtracking
    while i > 0 && j > 0 {
        println!("==================");
        for x in 0..=entries_a.len() {
            for y in 0..=entries_b.len() {
                if x == i && y == j {
                    print!("[{}] ", dp[x][y]);
                } else {
                    print!(" {}  ", dp[x][y]);
                }
            }
            println!();
        }
        println!("Cursor está em: (i={}, j={})", i, j);

        if entries_a[i-1] == entries_b[j-1] {
            println!("Match! '{}' adicionado à LCS", entries_a[i-1]);
            lcs.push(entries_a[i-1]);
            i -= 1;
            j -= 1;
        }
        else if dp[i-1][j] > dp[i][j-1] {
            println!("Movendo para cima (i-1)");
            i -= 1;
        }
        else {
            println!("Movendo para esquerda (j-1)");
            j -= 1;
        }
    }

    lcs.reverse();
    println!("==================");
    println!("LCS completa é: {:?}", lcs);
}

// Explicação
// Primeiro, criar e setar valores em uma matriz com o objetivo de obter o TAMANHO da lcs.
// 1. Inicializa uma matriz com os vetores de entrada (A e B), adicionando uma linha e coluna extra: vec![vec![0; entries_b.len() + 1]; entries_a.len() + 1];
// 2. Percorre a matriz apartir da linha 1 (i) e coluna 1 (j)
// 3. Para cada elemento percorrido, é verificado se são iguais (match): entries_a[i-1] == entries_b[j-1]
// 4. Se ocorreu match de igualdade, vamos setar o valor da celula percorrida (dp[i][j]) com o resultado dessa formula: dp[i][j] = dp[i-1][j-1] + 1
// 5. Se não ocorreu match de igualdade, vamos setar o valor da celula atual (dp[i][j]) com o resultado dessa formula: dp[i][j] = max(dp[i-1][j], dp[i][j-1])
// 6. Após todos esses passos, teremos uma matriz completa com todos os valores, e o cumprimento/tamanho da LCS está armazenado no canto inferiro direito (ultima linha, ultima coluna)

// Segundo, realizar o backtracking na matriz, iniciando pelo ultimo elemento da matriz (ultima linha e ultima coluna)
// 1. Inicializa duas variaveis contendo o cumprimento dos vetores de entrada: i = entries_a.len() e j = entries_b.len()
// 2. Para cada elemento percorrido, é verificado se são iguais (match): entries_a[i-1] == entries_b[j-1]
// 3. Se ocorreu match de igualdade, vamos adicionar esse elemento na lista de LCS e avançar para a proxima celula na diagonal da esquerda. (i - 1, j- 1) 
// 4. Se não ocorreu match de igualdade, vamos realizar uma operação lógica: dp[i-1][j] > dp[i][j-1]
// 5. Se a operação for true: vamos avançar para cima (i-1)
// 6. Se a operação for false: vamos avançar para a esquerda (-1)