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
