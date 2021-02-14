// Keep this definition alone in this file, it is included by the build script.

/// Information about a board piece, to place and display it.
#[derive(Debug, PartialEq, Eq)]
pub struct DisplayBoardPlacementInfo {
    /// How many columns to the right of the top_left do you need.
    pub width_right: u8,
    /// How many columns to the left of the top_left do you need.
    pub width_left: u8,
    /// How many rows below the top_left do you need.
    pub height: u8,

    /// The number of balls on the piece. Also the number of elements in the array.
    pub num_balls: u8,
    /// The index of the balls, relative to the top-left corner.
    pub balls: [u8; 6],
}
