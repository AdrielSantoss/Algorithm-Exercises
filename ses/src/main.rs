fn main() {
    let entries_a = vec!["A", "G", "G", "T", "A", "B"];
    let entries_b = vec!["G", "X", "T", "X", "A", "Y", "B"];

    let mut dp: Vec<Vec<i32>> = vec![vec![0; entries_b.len() + 1]; entries_a.len() + 1];

    for i in 1..=entries_a.len()  {
        dp[0][i] = i as i32;
    }

    for j in 1..=entries_b.len()  {
        dp[j][0] = j as i32;
    }
}
