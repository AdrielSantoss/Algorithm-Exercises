fn main() {
    println!("Resolvendo o problema do Myers!");

    let entries_a = vec!["A", "G", "G", "T", "A", "B"];
    let entries_b = vec!["G", "X", "T", "X", "A", "Y", "B"];

    let n = entries_a.len();
    let m = entries_b.len();

    let max = (n + m) as usize;
    let offset = max;

    let mut v: Vec<isize> = vec![0; 2 * max + 1];
    let mut trace: Vec<isize> = vec![0; 2 * max + 1];

    for d in 0..=max as isize {
        for k in (-d..=d).step_by(2) {
           let k_idx = (k + offset as isize) as usize;

           let mut x = 0;
           let mut y = 0;

           let delete = v[(k+1+offset as isize) as usize];
           let insert = v[(k-1+offset as isize) as usize] + 1;

            if k == -d {
                x = insert;
            } else if k == d {
                x = delete;
            }
            else {
                x = delete.max(insert);
            }

            y = x - k;

            v[k_idx] = x;

            while x < n as isize && y < m as isize && entries_a[x as usize] == entries_b[y as usize] {
                x += 1;
                y += 1;
            }

            if (x as usize) >= n && (y as usize) >= m {
                return;
            }
        }
    }
}
