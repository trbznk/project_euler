use std::fs;

const GRID_SIZE: usize   = 20;
const FILTER_SIZE: usize = 4;

fn comp_max_value(filter: [[u32; FILTER_SIZE]; FILTER_SIZE]) -> u32 {
    let mut value: u32 = 0;

    // rows
    for i in 0..FILTER_SIZE {
        let p = filter[i][0]*filter[i][1]*filter[i][2]*filter[i][3];
        if p > value {
            value = p;
        }
    }

    // cols
    for i in 0..FILTER_SIZE {
        let p = filter[0][i]*filter[1][i]*filter[2][i]*filter[3][i];
        if p > value {
            value = p;
        }
    }

    // diagonal tl -> br
    let p = filter[0][0]*filter[1][1]*filter[2][2]*filter[3][3];
    if p > value {
        value = p;
    }

    // diagonal bl -> tr
    let p = filter[3][0]*filter[2][1]*filter[1][2]*filter[0][3];
    if p > value {
        value = p;
    }

    value
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("ERROR: Cannot read input file");

    let mut grid = [[0u32; GRID_SIZE]; GRID_SIZE];
    for (i, row) in input.lines().enumerate() {
        for (j, col) in row.split_whitespace().enumerate() {
            grid[i][j] = col.parse().unwrap();
        }
    }

    let mut filter = [[0; FILTER_SIZE]; FILTER_SIZE];
    let mut max_value = 0;
    for i in 0..GRID_SIZE-FILTER_SIZE+1 {
        for j in 0..GRID_SIZE-FILTER_SIZE+1 {
            for ii in 0..FILTER_SIZE {
                for jj in 0..FILTER_SIZE {
                    filter[ii][jj] = grid[i+ii][j+jj];
                }
            }
            let value = comp_max_value(filter);
            if value > max_value {
                max_value = value;
            }
        }
    }

    println!("{}", max_value);
}

