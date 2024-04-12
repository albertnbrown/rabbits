use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// if you are unfamiliar with rust, run me via `cargo run path-to-file`

fn main() {
    // Args: path to input file
    // the lines are the comma delimited rows of the array
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // get file
    let file_name: &String = &args[1];
    let f = BufReader::new(File::open(file_name).unwrap());

    // read file into array
    let mut arr: Vec<Vec<usize>> = f.lines()
        .map(|l| l.unwrap().split(',')
            .map(|number| number.parse().unwrap())
            .collect())
        .collect();

    // n is the number of columns, m is the number of rows
    let n = arr[0].len();
    let m = arr.len();

    // check inputs
    println!("n:{} m:{}", n, m);
    println!("{:?}", arr);

    // finding the start square

    // flags for if we need to search for a center
    let mut col_flag: bool = false;
    let mut row_flag: bool = false;
    
    // initial guess of the center
    // x is the left to right position
    // y is the up to down position
    let mut x: usize= if (n&1 == 1) {
        (n+1)/2 - 1 // odd case, no searching
    } else {
        col_flag = true;
        n/2 - 1 // even case, searching
    };
    let mut y: usize = if (m&1 == 1) {
        (m+1)/2 - 1
    } else {
        row_flag = true;
        m/2 - 1
    };

    // println!("col:{} row:{}", col_flag, row_flag);
    // println!("x:{} y:{}", x, y);

    // finding the best center to start at
    let mut max_center: usize = arr[y][x];

    // if both are uneven, we need to check one more case
    let corner_flag: bool = col_flag&&row_flag;

    // checks right of center
    if (col_flag) {
        if (max_center < arr[y][x+1]) {
            max_center = arr[y][x+1];
        } else {
            col_flag = false;
        }
    }

    // checks down of center
    if (row_flag) {
        if (max_center < arr[y+1][x]) {
            max_center = arr[y+1][x];
            col_flag = false; // means that down of center beat out right of center
        } else {
            row_flag = false;
        }
    }

    // checks diagonal of center
    if (corner_flag) {
        if (max_center < arr[y+1][x+1]) {
            col_flag = true;
            row_flag = true;
        }
    }

    // increment if flagged to find the maxed center
    if (col_flag) {
        x += 1;
    }
    if (row_flag) {
        y += 1;
    }

    // println!("x:{} y:{}", x, y);

    // do the eating
    let mut eaten: usize = 0;
    loop {
        eaten += arr[y][x];
        arr[y][x] = 0;

        // tracks the max carrots in a direction
        // setting this to 0 by default means we solve the 1x1
        let mut max_surroundings: usize = 0;
        // tracks the direction with the most carrots
        let mut facing_surroundings: usize = 0;

        // check right
        if (x != n - 1) {
            max_surroundings = arr[y][x + 1];
            facing_surroundings = 1;
        }

        // check down
        if (y != m - 1) {
            if (arr[y+1][x] > max_surroundings){
                max_surroundings = arr[y+1][x];
                facing_surroundings = 2;
            }
        }

        // check left
        if (x != 0) {
            if (arr[y][x - 1] > max_surroundings){
                max_surroundings = arr[y][x - 1];
                facing_surroundings = 3;
            }
        }

        // check up
        if (y != 0) {
            if (arr[y - 1][x] > max_surroundings){
                max_surroundings = arr[y - 1][x];
                facing_surroundings = 4;
            }
        }

        // no carrots in surroundings, break
        if (max_surroundings == 0) {
            break;
        }

        // map the tastiest direction to a move
        match facing_surroundings {
            1 => {
                x += 1;
            }
            2 => {
                y += 1;
            }
            3 => {
                x -= 1;
            }
            4 => {
                y -= 1;
            }
            _ => {
                println!("don't think this ever gets hit");
                break;
            }
        }
    }

    println!("{}", eaten);
}
