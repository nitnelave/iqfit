use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn get_pieces() -> Vec<Vec<(u8, i8)>> {
    // They all have (0, 0), (1, 0), (2, 0)
    let yellow_a: Vec<(u8, i8)> = vec![(0, 1), (3, 0)];
    let yellow_b: Vec<(u8, i8)> = vec![(0, 1), (1, 1), (3, 0)];
    let orange_a: Vec<(u8, i8)> = vec![(3, 0), (2, 1)];
    let orange_b: Vec<(u8, i8)> = vec![(3, 0), (1, 1), (3, 1)];
    let red_a: Vec<(u8, i8)> = vec![(3, 0), (3, 1)];
    let red_b: Vec<(u8, i8)> = vec![(3, 0), (0, 1), (3, 1)];
    let pink_a: Vec<(u8, i8)> = vec![(3, 0), (1, 1)];
    let pink_b: Vec<(u8, i8)> = vec![(3, 0), (2, 1), (3, 1)];
    let light_green_a: Vec<(u8, i8)> = vec![(2, 1)];
    let light_green_b: Vec<(u8, i8)> = vec![(0, 1), (2, 1)];
    let green_a: Vec<(u8, i8)> = vec![(1, 1)];
    let green_b: Vec<(u8, i8)> = vec![(1, 1), (2, 1)];
    let light_blue_a: Vec<(u8, i8)> = vec![(3, 0), (2, 1)];
    let light_blue_b: Vec<(u8, i8)> = vec![(3, 0), (1, 1), (2, 1)];
    let blue_a: Vec<(u8, i8)> = vec![(3, 0), (3, 1)];
    let blue_b: Vec<(u8, i8)> = vec![(3, 0), (0, 1), (2, 1)];
    let deep_blue_a: Vec<(u8, i8)> = vec![(1, 1)];
    let deep_blue_b: Vec<(u8, i8)> = vec![(0, 1), (2, 1)];
    let purple_a: Vec<(u8, i8)> = vec![(0, 1)];
    let purple_b: Vec<(u8, i8)> = vec![(0, 1), (1, 1)];
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

fn main() -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("display_board_placement_info_gen.rs");
    let mut file = fs::File::create(&dest_path)?;
    file.write_all(
        br#"
use crate::board::DisplayBoardPlacementInfo;

pub const PLACEMENT_INFO: &[&DisplayBoardPlacementInfo; 80] = &[&DisplayBoardPlacementInfo {
    num_balls: 0,
    width_right: 0,
    width_left: 0,
    height: 0,
    balls: [0;6],
}; 80];
"#,
    )?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
