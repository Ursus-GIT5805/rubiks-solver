pub mod turn;
pub mod arraycube;

use turn::*;

const N: usize = 3;
pub const NUM_SIDES: usize = 6;

type Side = u8;

pub const UP: Side = 0;
pub const DOWN: Side = 1;
pub const BACK: Side = 2;
pub const FRONT: Side = 3;
pub const LEFT: Side = 4;
pub const RIGHT: Side = 5;
pub const UNKNOWN: Side = 6;

/// Returns the ANSI-colorcode for the given side.
pub fn get_color(side: Side) -> &'static str {
    match side {
	UP    => "\x1b[00m", // White
	DOWN  => "\x1b[93m", // Yellow
	BACK  => "\x1b[32m", // Green
	FRONT => "\x1b[34m", // Blue
	LEFT  => "\x1b[31m", // Red
	RIGHT => "\x1b[33m", // Orange
	_ => "\x1b[00m",
    }
}

// ---

/// All the different position names for an Edge
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(strum::EnumIter)]
#[derive(Debug)]
#[repr(usize)]
pub enum Edge {
    UF, UR, UB, UL,
    DF, DR, DB, DL,
    FR, BR, BL, FL,
}

impl Edge {
    /// Parse the edge from the given colors.
    /// Returns None if there exist no Edge with the given colors.
    pub fn parse_edge(col: [Side; 2]) -> Option<Self> {
	// Create a hash out of the color
	let hash = {
	    let mut m: usize = 0;
	    for c in col {
		m += 1 << c as usize;
	    }
	    m
	};

	// Each hash matches a different edge.
	let res = match hash {
	    0b_00_10_01 => Self::UF,
	    0b_10_00_01 => Self::UR,
	    0b_00_01_01 => Self::UB,
	    0b_01_00_01 => Self::UL,

	    0b_00_10_10 => Self::DF,
	    0b_10_00_10 => Self::DR,
	    0b_00_01_10 => Self::DB,
	    0b_01_00_10 => Self::DL,

	    0b_10_10_00 => Self::FR,
	    0b_01_10_00 => Self::FL,
	    0b_01_01_00 => Self::BL,
	    0b_10_01_00 => Self::BR,
	    _ => return None,
	};

	Some(res)
    }
}

/// A corner piece
/// Note that the name is carefully sorted!
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(strum::EnumIter)]
#[derive(Debug)]
#[repr(usize)]
pub enum Corner {
    URF, UBR, DLF, DFR, // DON'T CHANGE THE ORDER OF THE LETTERS!
    ULB, UFL, DRB, DBL,
}

impl Corner {
    /// Parse the corner from the given colors.
    /// If no corner with the given colors exist, the function returns None.
    pub fn parse_corner(col: [Side; 3]) -> Option<Self> {
	// Create a hash
	let hash = {
	    let mut m: usize = 0;
	    for c in col {
		m += 1 << c as usize;
	    }
	    m
	};

	// Match the hash
	let res = match hash {
	    // Note that
	    // AA checks for left/right
	    // BB for front/back
	    // CC for up/down
	    //
	    // AA_BB_CC
	    0b_01_10_01 => Self::UFL,
	    0b_10_10_01 => Self::URF,
	    0b_01_01_01 => Self::ULB,
	    0b_10_01_01 => Self::UBR,
	    0b_01_10_10 => Self::DLF,
	    0b_10_10_10 => Self::DFR,
	    0b_01_01_10 => Self::DBL,
	    0b_10_01_10 => Self::DRB,
	    _ => return None,
	};

	Some(res)
    }
}


/// The RubiksCube trait.
///
pub trait RubiksCube {
    fn apply_turn(&mut self, turn: Turn);
}
