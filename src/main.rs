use rand::Rng;

fn main() {
    let dimensions = term_size::dimensions().unwrap();

    let mut grid: Vec<Vec<bool>> = vec![vec![false; dimensions.0]; dimensions.1];

    let mut rng = rand::thread_rng();

    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            grid[y][x] = rng.gen();
        }
    }

    loop {
        // Print the grid
        let mut output_str = String::from("");

        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                if grid[y][x] {
                    output_str += "M";
                } else {
                    output_str += " ";
                }
            }
        }

        print!("{}", output_str);

        // Next generation

        let mut new_grid = grid.clone();

        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                let y = y as usize;
                let x = x as usize;

                // Get neigbhoor cells
                let neighboor_lines: Vec<Vec<bool>>;

                if y == 0 {
                    neighboor_lines = grid[y..y + 2].to_vec();
                } else if y == dimensions.1 - 1 {
                    neighboor_lines = grid[y - 1..y + 1].to_vec();
                } else {
                    neighboor_lines = grid[y - 1..y + 2].to_vec();
                }

                let mut neighboors = neighboor_lines.clone();

                for i in 0..neighboor_lines.len() {
                    if x == 0 {
                        neighboors[i] = neighboor_lines[i][x..x + 2].to_vec();
                    } else if x == dimensions.0 - 1 {
                        neighboors[i] = neighboor_lines[i][x - 1..x + 1].to_vec();
                    } else {
                        neighboors[i] = neighboor_lines[i][x - 1..x + 2].to_vec();
                    }
                }

                // Count number of alive negihboors
                let mut num_neighboors = 0;

                for neighboor in neighboors.into_iter().flatten().collect::<Vec<bool>>() {
                    if neighboor {
                        num_neighboors += 1;
                    }
                }

                // Update cell in next grid
                if num_neighboors == 3 {
                    new_grid[y][x] = true;
                } else if num_neighboors != 2 {
                    new_grid[y][x] = false;
                }
            }
        }

        grid = new_grid;

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
