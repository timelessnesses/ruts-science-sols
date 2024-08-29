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
        let s = input(Some(
            "Input the start, ending and building height change split by space: ",
        ));
        let mut parsed = s.split(" ").map(|i| i.parse::<isize>().unwrap());
        let start = parsed.next().unwrap();
        let end = parsed.next().unwrap();
        let height_change = parsed.next().unwrap();
        for x in start - 1..end {
            BUILDINGS[x as usize] += height_change;
        }
        let max_height = *BUILDINGS.iter().max().unwrap();
        let mut current_count = 0;
        let mut max_near_buildings_count = 0;
        for b in BUILDINGS.iter() {
            if *b == max_height { current_count += 1; }
            else {
                if current_count > max_near_buildings_count { max_near_buildings_count = current_count; }
                current_count = 0;
            }
        }
        println!("Max height: {max_height}, Tallest near buildings count: {max_near_buildings_count}");
    }
}
