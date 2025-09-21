fn main() {
    println!("Resolvendo o problema do Myers!");

    let entries_a = vec!["A", "G", "G", "T", "A", "B"];
    let entries_b = vec!["G", "X", "T", "X", "A", "Y", "B"];

    let n = entries_a.len();
    let m = entries_b.len();

    let max = (n + m) as usize;
    let offset = max;

    let mut v: Vec<isize> = vec![0; 2 * max + 1];

    for d in 0..=max as isize {
        for k in (-d..=d).step_by(2) {
           let k_idx = (k + offset as isize) as usize;

            let mut x = if k == -d || (k != d && v[(k-1+offset as isize) as usize] < v[(k+1+offset as isize) as usize])
            {
                v[(k+1+offset as isize) as usize]
            } else {
                v[(k-1+offset as isize) as usize] + 1
            };

            let mut y = x - k;

            while x < n as isize && y < m as isize && entries_a[x as usize] == entries_b[y as usize] {
                x += 1;
                y += 1;
            }

            v[k_idx] = x;

            if (x as usize) >= n && (y as usize) >= m {
                println!("O tamanho da SES é: {}", d);
                return;
            }
        }
    }
}
