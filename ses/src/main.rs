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

    println!("Distância de edição (SES): {}", dp[entries_a.len()][entries_b.len()]);
}
