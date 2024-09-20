use std::collections::HashMap;

fn main() {
    //grab from stdin
    loop {
        let mut a = String::new();
        println!("Enter a string: ");
        std::io::stdin().read_line(&mut a).unwrap();
        let mut b = String::new();
        println!("Enter another string: ");
        std::io::stdin().read_line(&mut b).unwrap();

        let a: Vec<char> = a.trim().chars().collect();
        let b: Vec<char> = b.trim().chars().collect();

        let (res, comparisons, assignments) = lcs(a.clone(), b.clone());

        println!("\nSolution:");

        // print a with lcs indices colored green
        for (i, char) in a.iter().enumerate() {
            if res.a_indices.contains(&i) {
                print!("\x1b[32m{}\x1b[0m", char);
            } else {
                print!("{}", char);
            }
        }
        println!();
        // print b with indices colored green
        for (i, char) in b.iter().enumerate() {
            if res.b_indices.contains(&i) {
                print!("\x1b[32m{}\x1b[0m", char);
            } else {
                print!("{}", char);
            }
        }
        println!();
        println!();
        println!("Comparisons: {}", comparisons);
        println!("Assignments: {}", assignments);
        println!();
    }
}

struct Res {
    solution: String,
    a_indices: Vec<usize>,
    b_indices: Vec<usize>,
}

fn lcs(a: Vec<char>, b: Vec<char>) -> (Res, usize, usize) {
    let mut length_table: HashMap<(isize, isize), usize> = HashMap::new();
    let mut breadcrumbs: HashMap<(isize, isize), Option<(isize, isize)>> = HashMap::new();
    let mut comparisons = 0;
    let mut assignments = 0;

    for a_i in -1..(a.len() as isize) {
        for b_i in -1..(b.len() as isize) {
            comparisons += 2; // for the two if conditions
            if (a_i == -1) || (b_i == -1) {
                assignments += 2;
                length_table.insert((a_i, b_i), 0);
                breadcrumbs.insert((a_i, b_i), None);
                continue;
            }

            let l = length_table.get(&(a_i - 1, b_i)).unwrap_or(&0);
            let u = length_table.get(&(a_i, b_i - 1)).unwrap_or(&0);
            comparisons += 1; // for the equality check
            if a[a_i as usize] == b[b_i as usize] {
                let ul = length_table.get(&(a_i - 1, b_i - 1)).unwrap_or(&0);
                let longest = *[ul, l, u].iter().max().unwrap();
                assignments += 2;
                length_table.insert((a_i, b_i), longest + 1);
                breadcrumbs.insert((a_i, b_i), Some((a_i - 1, b_i - 1)));
            } else {
                comparisons += 1; // for the l > u check
                if l > u {
                    assignments += 2;
                    length_table.insert((a_i, b_i), *l);
                    breadcrumbs.insert((a_i, b_i), Some((a_i - 1, b_i)));
                } else {
                    assignments += 2;
                    length_table.insert((a_i, b_i), *u);
                    breadcrumbs.insert((a_i, b_i), Some((a_i, b_i - 1)));
                }
            }
        }
    }

    // Reconstruct the LCS with indices
    let mut result = Vec::new();
    let mut current = (a.len() as isize - 1, b.len() as isize - 1);

    while let Some(prev) = breadcrumbs.get(&current).unwrap_or(&None) {
        if prev.0 == current.0 - 1 && prev.1 == current.1 - 1 {
            result.push((
                a[current.0 as usize],
                current.0 as usize,
                current.1 as usize,
            ));
        }
        current = *prev;
    }

    let mut solution = String::new();
    let mut a_indices = Vec::new();
    let mut b_indices = Vec::new();

    for (c, a_i, b_i) in result {
        solution.push(c);
        a_indices.push(a_i);
        b_indices.push(b_i);
    }

    (
        Res {
            solution,
            a_indices,
            b_indices,
    },
        comparisons,
        assignments,
    )
}
