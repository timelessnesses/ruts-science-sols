use utils::input;

fn clamp(n: isize, min: isize, max: isize) -> isize {
    if n > max {
        max
    } else if n < min {
        min
    } else {
        n
    }
}

fn main() {
    #[allow(non_snake_case)]
    let N = input(Some("Plots: ")).parse::<usize>().unwrap();
    #[allow(non_snake_case)]
    let M = input(Some("Developments: ")).parse::<usize>().unwrap();

    #[allow(non_snake_case)]
    let mut BUILDINGS: Vec<isize> = Vec::with_capacity(N);
    for _ in 0..N {
        BUILDINGS.push(0);
    }
    for _ in 0..M {
        let s = input(Some("Input the start, ending and building height change split by space: "));
        let mut parsed = s.split(" ").map(|i| i.parse::<isize>().unwrap());
        let start = parsed.next().unwrap();
        let end = parsed.next().unwrap();
        let height_change = parsed.next().unwrap();
        for x in start - 1..end{
            BUILDINGS[x as usize] += height_change;
        }
        let max_height = *BUILDINGS.iter().max().unwrap();
        let mut tallest_near_buildings_together_count = 0;
        for (i, b) in BUILDINGS.iter().enumerate() {
            if *b == max_height && (
                BUILDINGS[clamp(i as isize - 1, 0, N as isize - 1) as usize] == max_height || BUILDINGS[clamp(i as isize + 1, 0, N as isize - 1) as usize] == max_height
            ) {
                tallest_near_buildings_together_count += 1;
            }
        }
        println!("Max height: {max_height}, Tallest near buildings count: {tallest_near_buildings_together_count}");
        println!("{:#?}", BUILDINGS);
    }
}