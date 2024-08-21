use utils::input;

type Matrix = Vec<Vec<i128>>;

fn main() {
    #[allow(non_snake_case)]
    let M = input(Some("Rows: ")).parse::<usize>().unwrap();
    #[allow(non_snake_case)]
    let N = input(Some("Columns: ")).parse::<usize>().unwrap();
    let mut first_matrix = Vec::new();
    let mut second_matrix = Vec::new();
    println!("Enter first matrix values ({M}x{N})");
    setup_matrix(M, N, &mut first_matrix);
    println!("Enter second matrix values ({M}x{N})");
    setup_matrix(M, N, &mut second_matrix);
    let mut result: Matrix = Vec::new();
    for x in 0..M {
        for y in 0..N {
            if result.len() != x + 1 {
                result.push(Vec::new());
            }
            if result[x].len() != y + 1 {
                result[x].push(0);
            }
            result[x][y] = first_matrix[x][y] + second_matrix[x][y];
        }
    }
    println!("Result");
    for x in result {
        let out = x
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("     ");
        println!("{}", out);
    }
}

fn setup_matrix(row: usize, column: usize, a: &mut Matrix) {
    for x in 0..row {
        for y in 0..column {
            if a.len() != x + 1 {
                a.push(Vec::new());
            }
            if a[x].len() != y + 1 {
                a[x].push(0);
            }
            a[x][y] = input(Some(&format!("Enter number [{}][{}]: ", x + 1, y + 1)))
                .parse::<i128>()
                .expect("Failed to parse integer");
        }
    }
}
