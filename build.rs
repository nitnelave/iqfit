use std::env;
use std::fs;
use std::path::Path;

include!("src/board/display_board_placement_info.rs");

fn get_pieces() -> Vec<Vec<(i8, i8)>> {
    // They all have (0, 0), (1, 0), (2, 0)
    let yellow_a: Vec<(i8, i8)> = vec![(0, 1), (3, 0)];
    let yellow_b: Vec<(i8, i8)> = vec![(0, 1), (1, 1), (3, 0)];
    let orange_a: Vec<(i8, i8)> = vec![(3, 0), (2, 1)];
    let orange_b: Vec<(i8, i8)> = vec![(3, 0), (1, 1), (3, 1)];
    let red_a: Vec<(i8, i8)> = vec![(3, 0), (3, 1)];
    let red_b: Vec<(i8, i8)> = vec![(3, 0), (0, 1), (3, 1)];
    let pink_a: Vec<(i8, i8)> = vec![(3, 0), (1, 1)];
    let pink_b: Vec<(i8, i8)> = vec![(3, 0), (2, 1), (3, 1)];
    let light_green_a: Vec<(i8, i8)> = vec![(2, 1)];
    let light_green_b: Vec<(i8, i8)> = vec![(0, 1), (2, 1)];
    let green_a: Vec<(i8, i8)> = vec![(1, 1)];
    let green_b: Vec<(i8, i8)> = vec![(1, 1), (2, 1)];
    let light_blue_a: Vec<(i8, i8)> = vec![(3, 0), (2, 1)];
    let light_blue_b: Vec<(i8, i8)> = vec![(3, 0), (1, 1), (2, 1)];
    let blue_a: Vec<(i8, i8)> = vec![(3, 0), (3, 1)];
    let blue_b: Vec<(i8, i8)> = vec![(3, 0), (0, 1), (2, 1)];
    let deep_blue_a: Vec<(i8, i8)> = vec![(1, 1)];
    let deep_blue_b: Vec<(i8, i8)> = vec![(0, 1), (2, 1)];
    let purple_a: Vec<(i8, i8)> = vec![(0, 1)];
    let purple_b: Vec<(i8, i8)> = vec![(0, 1), (1, 1)];
    vec![
        yellow_a,
        yellow_b,
        orange_a,
        orange_b,
        red_a,
        red_b,
        pink_a,
        pink_b,
        light_green_a,
        light_green_b,
        green_a,
        green_b,
        light_blue_a,
        light_blue_b,
        blue_a,
        blue_b,
        deep_blue_a,
        deep_blue_b,
        purple_a,
        purple_b,
    ]
}

/// Rotate a piece, and shift it so the top-left corner is at (0, 0).
fn rotate_piece(balls: &mut Vec<(i8, i8)>) {
    for b in balls.iter_mut() {
        let tmp = -b.0;
        b.0 = b.1;
        b.1 = tmp;
    }
    balls.sort_unstable();
    let top_left = balls[0];
    for b in balls.iter_mut() {
        b.0 -= top_left.0;
        b.1 -= top_left.1;
    }
    assert!(balls[0] == (0, 0));
}

fn coord_to_index(coords: (i8, i8)) -> u8 {
    assert!(coords.0 >= 0, "Negative row!");
    (coords.0 * 10 + coords.1) as u8
}

fn coords_to_binary(coords: &[u8]) -> u64 {
    let mut res = 0;
    for c in coords {
        res |= 1 << c;
    }
    res
}

fn get_display_info(piece: &[(i8, i8)]) -> DisplayBoardPlacementInfo {
    let max_row = piece.iter().map(|b| b.0).max().unwrap();
    let min_col = piece.iter().map(|b| b.1).min().unwrap();
    let max_col = piece.iter().map(|b| b.1).max().unwrap();
    assert!(min_col <= 0, "Got positive min_col: {}", min_col);
    assert!(max_col >= 0, "Got negative max_col: {}", max_col);
    assert!(max_row >= 0, "Got negative max_row: {}", max_row);
    let mut balls: [u8; 6] = [0; 6];
    for i in 0..piece.len() {
        balls[i] = coord_to_index(piece[i]);
    }
    DisplayBoardPlacementInfo {
        width_right: max_col as u8,
        width_left: (-min_col) as u8,
        height: max_row as u8,
        num_balls: piece.len() as u8,
        balls,
        as_binary: coords_to_binary(&balls),
    }
}

fn write_piece<T: std::io::Write>(file: &mut T, piece: &[(i8, i8)]) {
    let info = get_display_info(piece);
    writeln!(file, "    &{:#?},", info).unwrap();
}

fn write_pieces<T: std::io::Write>(file: &mut T) {
    write!(
        file,
        "
use crate::board::DisplayBoardPlacementInfo;

pub const PLACEMENT_INFO: &[&DisplayBoardPlacementInfo; 80] = &["
    )
    .unwrap();

    for p in get_pieces().iter_mut() {
        for i in 0..3 {
            p.push((i, 0));
        }
        for _ in 0..4 {
            write_piece(file, p);
            rotate_piece(p);
        }
    }
    write!(file, "];").unwrap();
}

fn first_unset_bit(byte: u8) -> u8 {
    for i in 0..8 {
        if byte & (1 << i) == 0 {
            return i;
        }
    }
    8
}

fn write_first_unset_bit_table<T: std::io::Write>(file: &mut T) {
    write!(file, "pub const FIRST_UNSET_BIT: [u8; 256] = [").unwrap();

    for byte in 0..=255 {
        write!(
            file,
            "
    {},",
            first_unset_bit(byte)
        )
        .unwrap();
    }
    write!(
        file,
        "
];"
    )
    .unwrap();
}

fn main() -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("display_board_placement_info_gen.rs");
    let mut file = fs::File::create(&dest_path)?;
    write_pieces(&mut file);
    let dest_path = Path::new(&out_dir).join("first_unset_bit_table.rs");
    let mut file = fs::File::create(&dest_path)?;
    write_first_unset_bit_table(&mut file);
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
