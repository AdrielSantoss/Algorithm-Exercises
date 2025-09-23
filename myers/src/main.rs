fn main() {
    println!("Resolvendo o problema do Myers!");

    let entries_a = vec!["A", "G", "G", "T", "A", "B"];
    let entries_b = vec!["G", "X", "T", "X", "A", "Y", "B"];

    let n = entries_a.len();
    let m = entries_b.len();

    let max = (n + m) as usize;
    let offset = max;

    let mut v: Vec<isize> = vec![0; 2 * max + 1];
    let mut trace: Vec<Vec<isize>> = Vec::new();

    for d in 0..=max as isize {
        let mut y = 0;
        let mut x = 0;

        for k in (-d..=d).step_by(2) {
           let k_idx = (k + offset as isize) as usize; 

            x = if k == -d || (k != d && v[(k-1+offset as isize) as usize] < v[(k+1+offset as isize) as usize])
            {
                v[(k+1+offset as isize) as usize]
            } else {
                v[(k-1+offset as isize) as usize] + 1
            };

            y = x - k;

            while x < n as isize && y < m as isize && entries_a[x as usize] == entries_b[y as usize] {
                x += 1;
                y += 1;
            }

            v[k_idx] = x;

            if (x as usize) >= n && (y as usize) >= m {
                println!("O tamanho da SES é: {}", d);
                trace.push(v.clone()); 
                break;
            }
        }

        if (x as usize) >= n && (y as usize) >= m {
            break;
        }

        trace.push(v.clone());
    }

    let mut x = n as isize;
    let mut y = m as isize;
    let mut edits: Vec<String> = Vec::new();

    for (d, v) in trace.iter().enumerate().rev() {
        let k = x - y;

        let d_isize = d as isize;

        let prev_k = if k == -d_isize
            || (k != d_isize && v[(k - 1 + offset as isize) as usize] < v[(k + 1 + offset as isize) as usize])
        {
            k + 1
        } else {
            k - 1
        };

        let prev_x = v[(prev_k + offset as isize) as usize];
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y {
            edits.push(format!("MATCH {}", entries_a[(x - 1) as usize]));

            x -= 1;
            y -= 1;
        }

        if d > 0 {
            if y > 0 && x == prev_x {
                edits.push(format!("INSERT {}", entries_b[(y - 1) as usize]));

                y -= 1;
            } else if x > 0 {
                edits.push(format!("DELETE {}", entries_a[(x - 1) as usize]));

                x -= 1;
            }
        }
    }

    for op in edits.iter().rev() {
        print!("{}", op)
    }
}
