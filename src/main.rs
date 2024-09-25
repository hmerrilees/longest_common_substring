use std::io::{self};

struct DPEntry {
    score: usize,
    breadcrumb: BreadcrumbType,
    in_solution: bool, // bookkeeping to print green answer
}

fn lcs(a: &[char], b: &[char]) -> Res {
    use BreadcrumbType::*;

    let (m, n) = (a.len(), b.len());
    let mut dp = vec![Vec::with_capacity(n + 1); m + 1];
    let mut breadcrumbs = vec![Vec::with_capacity(n + 1); m + 1];

    // Base cases
    // First row
    for i in 0..=m {
        breadcrumbs[i].push(Breadcrumb::new(None));
        dp[i].push(0);
    }
    // First col... we already got the first row, start at 1
    for j in 1..=n {
        breadcrumbs[0].push(Breadcrumb::new(None));
        dp[0].push(0);
    }

    // Recursive cases, build the table top-down, left-to-right
    for (i, char_a) in a.iter().enumerate().map(|(i, &c)| (i + 1, c)) {
        for (j, char_b) in b.iter().enumerate().map(|(j, &c)| (j + 1, c)) {
            if char_a == char_b {
                let prev = dp[i - 1][j - 1]; // char match... discard both
                dp[i].push(prev + 1);
                breadcrumbs[i].push(Breadcrumb::new(Diagonal));
            } else if dp[i - 1][j] > dp[i][j - 1] {
                let prev = dp[i - 1][j]; // discard char_a
                dp[i].push(prev);
                breadcrumbs[i].push(Breadcrumb::new(Up));
            } else {
                // Note that breadcrumbs bias when the up/left cells are equal but we could have chosen to bias up too!
                let prev = dp[i][j - 1]; // discard char_b
                dp[i].push(prev);
                breadcrumbs[i].push(Breadcrumb::new(Left));
            };
        }
    }

    let mut i = m;
    let mut j = n;
    let mut solution_string = String::new();
    let mut a_indices = Vec::new();
    let mut b_indices = Vec::new();

    while i > 0 && j > 0 {
        let bc = &mut breadcrumbs[i][j];
        bc.in_solution = true;
        match bc {
            Breadcrumb {
                r#type: Diagonal, ..
            } => {
                solution_string.push(a[i - 1]);

                // some bookeeping for in-context solution printing
                a_indices.push(i - 1);
                b_indices.push(j - 1);

                // move diagonally
                i -= 1;
                j -= 1;
            }
            Breadcrumb { r#type: Up, .. } => {
                i -= 1; // move up
            }
            Breadcrumb { r#type: Left, ..  } => {
                j -= 1; // move left
            }
            _ => unreachable!(),
        }
    }

    // we collected
    solution_string = solution_string.chars().rev().collect();
    a_indices.reverse();
    b_indices.reverse();

    Res {
        solution: solution_string,
        a_indices,
        b_indices,
        dp,
        breadcrumbs,
    }
}

fn main() {
    loop {
        let a = read_input("Enter a string: ");
        let b = read_input("Enter another string: ");

        let Res {
            solution, a_indices, b_indices, dp, breadcrumbs
        } = lcs(&a, &b);

        println!("\nSolution:\n {solution}");
        println!("In context:");
        print_with_color(&a, &a_indices);
        print_with_color(&b, &b_indices);

        println!("DP Table:");
        for row in dp {
            println!("{:?}", row);
        }
        println!();

        println!("Breadcrumbs:");
        for row in breadcrumbs {
            for bc in row {
                print!("{}", bc);
            }
            println!();
        }

        let width = dp.last().unwrap().last().unwrap().to_string().len();

        println!();
        println!("Interleaved:");
        for (dp_row, breadcrumb_row) in dp.iter().zip(breadcrumbs.iter()) {
            // print vertical arrows
            for bc in breadcrumb_row.iter() {
                // min width of 2, ensure space before up arrow and after diagonal arrow to 
                match bc.r#type {
                    BreadcrumbType::Up => print!(" {bc:<width$}"),
                    BreadcrumbType::Diagonal => print!("{bc:<width$} "),
                    _ => print!("{:w$}", "", w=width+1)
                }
            }
            println!();
            // print dp entries and left arrows
            for (dp, breadcrumb) in dp_row.iter().zip(breadcrumb_row.iter()) {
                // left arrow
                if matches!(breadcrumb.r#type, BreadcrumbType::Left) {
                    print!("{breadcrumb}{:<width$}", dp, width = width);
                } else {
                    print!(" {:<width$}", dp, width = width);
                }
            }
            println!();
        }
        println!();
    }
}

struct Res {
    solution: String,
    a_indices: Vec<usize>,
    b_indices: Vec<usize>,
    dp: Vec<Vec<usize>>,
    breadcrumbs: Vec<Vec<Breadcrumb>>,
}

#[derive(Debug, Clone)]
struct Breadcrumb {
    r#type: BreadcrumbType,
    in_solution: bool,
}

impl Breadcrumb {
    fn new(r#type: BreadcrumbType) -> Self {
        Breadcrumb {
            r#type,
            in_solution: false, // should we make it green
        }
    }
}

#[derive(Debug, Clone)]
enum BreadcrumbType {
    Up,
    Left,
    Diagonal,
    None,
}

impl Default for BreadcrumbType {
    fn default() -> Self {
        BreadcrumbType::None
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


// Utilities

fn read_input(prompt: &str) -> Vec<char> {
    print!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
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
