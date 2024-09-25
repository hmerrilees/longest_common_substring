use std::io::{self};
struct Breadcrumb {
    r#type: BreadcrumbType,
    in_solution: bool,
}


#[derive(Clone)]
enum BreadcrumbType {
    Up,
    Left,
    Diagonal,
    None,
}

#[derive(Clone)]
struct DPEntry {
    score: usize,
    breadcrumb: BreadcrumbType,
    in_solution: bool,
}

impl DPEntry {
    fn new(score: usize, breadcrumb: BreadcrumbType) -> Self {
        DPEntry {
            score,
            breadcrumb,
            in_solution: false,
        }
    }
}

struct LCSResult {
    solution: String,
    a_indices: Vec<usize>,
    b_indices: Vec<usize>,
    dp_table: Vec<Vec<DPEntry>>,
}

fn lcs(a: &[char], b: &[char]) -> LCSResult {
    use BreadcrumbType::*;

    let (a_size, b_size) = (a.len(), b.len());
    let mut dp_table = vec![vec![DPEntry::new(0, None); b_size + 1]; a_size + 1];

    // Recursive cases, build the table top-down, left-to-right
    for (i, char_a) in a.iter().enumerate().map(|(i, &c)| (i + 1, c)) {
        for (j, char_b) in b.iter().enumerate().map(|(j, &c)| (j + 1, c)) {
            if char_a == char_b { // match
                let prev = dp_table[i - 1][j - 1].score;
                dp_table[i][j] = DPEntry::new(prev + 1, Diagonal);
            } else if dp_table[i - 1][j].score > dp_table[i][j - 1].score { // discard from a
                let prev = dp_table[i - 1][j].score;
                dp_table[i][j] = DPEntry::new(prev, Up);
            } else {
                let prev = dp_table[i][j - 1].score; // discard from b
                dp_table[i][j] = DPEntry::new(prev, Left);
            }
        }
    }

    let mut solution_string = String::new();
    let mut a_indices = Vec::new();
    let mut b_indices = Vec::new();

    // start in the bottom right corner (end of both strings)
    let mut a_i = a_size;
    let mut b_i = b_size;

    // walk back up the breadcrumbs
    while a_i > 0 && b_i > 0 {
        let entry = &mut dp_table[a_i][b_i];
        entry.in_solution = true;
        match entry.breadcrumb {
            Diagonal => {
                solution_string.push(a[a_i - 1]);
                // bookkeeping for in-context solution
                a_indices.push(a_i - 1);
                b_indices.push(b_i - 1);

                a_i -= 1;
                b_i -= 1;
            }
            Up => a_i -= 1,
            Left => b_i -= 1,
            None => break,
        }
    }

    solution_string = solution_string.chars().rev().collect();
    a_indices.reverse();
    b_indices.reverse();

    LCSResult {
        solution: solution_string,
        a_indices,
        b_indices,
        dp_table,
    }
}

fn main() {
    loop {
        let a = read_input("Enter the first string: ");
        let b = read_input("Enter the second string: ");

        let LCSResult {
            solution,
            a_indices,
            b_indices,
            dp_table,
        } = lcs(&a, &b);

        let max_score_digits = dp_table.last().unwrap().last().unwrap().score.to_string().len();

        println!("\nSolution:\n{solution}\n");
        println!("In context:");
        print_with_color(&a, &a_indices);
        print_with_color(&b, &b_indices);
        println!();

        println!("DP Table:");
        for row in &dp_table {
            for entry in row {
                print!("{:<max_score_digits$} ", entry.score);
            }
            println!();
        }
        println!();

        println!("Breadcrumbs:");
        for row in &dp_table {
            for entry in row {
                print!("{} ", Breadcrumb::new(entry.breadcrumb.clone(), entry.in_solution));
            }
            println!();
        }
        println!();

        println!("Interleaved:");
        for row in &dp_table {
            // print vertical arrows
            for entry in row {
                match entry.breadcrumb {
                    BreadcrumbType::Up => print!(" {:<max_score_digits$}", Breadcrumb::new(entry.breadcrumb.clone(), entry.in_solution)),
                    BreadcrumbType::Diagonal => print!("{:<max_score_digits$} ", Breadcrumb::new(entry.breadcrumb.clone(), entry.in_solution)),
                    _ => print!("{:w$}", "", w=max_score_digits+1)
                }
            }
            println!();
            // print dp entries and left arrows
            for entry in row {
                if matches!(entry.breadcrumb, BreadcrumbType::Left) {
                    print!("{}{:<max_score_digits$}", Breadcrumb::new(entry.breadcrumb.clone(), entry.in_solution), entry.score);
                } else {
                    print!(" {:<max_score_digits$}", entry.score);
                }
            }
            println!();
        }
        println!();
    }
}


// Utilities
impl Breadcrumb {
    fn new(r#type: BreadcrumbType, in_solution: bool) -> Self {
        Breadcrumb {
            r#type,
            in_solution,
        }
    }
}

impl std::fmt::Display for Breadcrumb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.r#type {
            BreadcrumbType::Up => '↑',
            BreadcrumbType::Left => '←',
            BreadcrumbType::Diagonal => '↖',
            BreadcrumbType::None => '·',
        };

        let s = if self.in_solution {
            format!("\x1b[32m{}\x1b[0m", c)
        } else {
            c.to_string()
        };

        f.pad(&s)
    }
}


fn read_input(prompt: &str) -> Vec<char> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!();
    input.trim().chars().collect()
}

fn print_with_color(s: &[char], indices: &[usize]) {
    for (i, &c) in s.iter().enumerate() {
        if indices.contains(&i) {
            print!("\x1b[32m{}\x1b[0m", c);
        } else {
            print!("{}", c);
        }
    }
    println!();
}
