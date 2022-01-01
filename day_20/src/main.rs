use std::fs;
use std::time::Instant;

fn parse_input(input: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    let (cipher, image) = input.split_once("\n\r").unwrap();

    (
        cipher.chars().map(|c| c != '.').collect(),
        image.trim().lines().fold(Vec::new(), |mut img, line| {
            img.push(line.chars().map(|c| c != '.').collect());
            img
        }),
    )
}

fn binary_to_decimal(binary: String) -> usize {
    usize::from_str_radix(&binary, 2).unwrap()
}

fn pad_image(matrix: &[Vec<bool>], pixel: bool) -> Vec<Vec<bool>> {
    let len = matrix[0].len() + 2;
    let mut expanded = vec![vec![pixel; len]];

    matrix.iter().for_each(|line| {
        let mut tmp = vec![pixel];
        tmp.extend(line);
        tmp.push(pixel);
        expanded.push(tmp);
    });

    expanded.extend(vec![vec![pixel; len]]);
    expanded
}

fn find_index(row: usize, col: usize, matrix: &[Vec<bool>], pixel: bool) -> usize {
    let mut neighbours = [pixel; 9];
    neighbours[4] = matrix[row][col];

    if row > 0 && col > 0 {
        neighbours[0] = matrix[row - 1][col - 1];
    }
    if row > 0 {
        neighbours[1] = matrix[row - 1][col];
    }
    if row > 0 && col + 1 < matrix[row].len() {
        neighbours[2] = matrix[row - 1][col + 1];
    }
    if col > 0 {
        neighbours[3] = matrix[row][col - 1];
    }
    if col + 1 < matrix[row].len() {
        neighbours[5] = matrix[row][col + 1];
    }
    if row + 1 < matrix.len() && col > 0 {
        neighbours[6] = matrix[row + 1][col - 1];
    }
    if row + 1 < matrix.len() {
        neighbours[7] = matrix[row + 1][col];
    }
    if row + 1 < matrix.len() && col + 1 < matrix[row].len() {
        neighbours[8] = matrix[row + 1][col + 1];
    }

    binary_to_decimal(
        neighbours
            .iter()
            .map(|c| if *c { '1' } else { '0' })
            .collect::<String>(),
    )
}

fn enhance((cipher, image): (Vec<bool>, Vec<Vec<bool>>), steps: usize) -> usize {
    let mut img = image;
    let mut i = 0;
    loop {
        let pixel = i % 2 != 0;

        img = pad_image(&img, pixel);
        let mut enhanced = img.clone();

        for x in 0..img.len() {
            for y in 0..img[x].len() {
                enhanced[x][y] = cipher[find_index(x, y, &img, pixel)];
            }
        }

        img = enhanced;

        i += 1;
        if i == steps {
            break;
        }
    }

    count_lit_pixels(&img)
}

fn count_lit_pixels(image: &[Vec<bool>]) -> usize {
    image.iter().flatten().filter(|p| **p).count()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let parsed = parse_input(&contents);

    let now = Instant::now();
    println!("{:?}", enhance(parsed, 50));
    println!("time: {}", now.elapsed().as_millis());
}
