fn main() {
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
}
