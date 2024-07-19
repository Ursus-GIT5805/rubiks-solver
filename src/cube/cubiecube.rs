use arraycube::{corner_to_indices, edge_to_indices};
use const_for::const_for;
use strum::IntoEnumIterator;

use crate::{
	cube::*,
	math::*,
};

pub type Ori = u32; // and the blind forest

type CornerList = [(Corner, Ori); NUM_CORNERS];
type EdgeList = [(Edge, Ori); NUM_EDGES];

/// The cube specification as Kociemba published in
/// https://kociemba.org/math/cubielevel.htm
///
/// Uses more space than necessary, but gives
/// very good insights about the cube's properties.
#[derive(PartialEq, Eq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct CubieCube {
	pub corners: CornerList,
	pub edges: EdgeList,
}

// ===== Tranformation-Corners =====
const TC_BASE: CornerList = [
	(Corner::URF, 0), (Corner::UBR, 0), (Corner::DLF, 0), (Corner::DFR, 0),
	(Corner::ULB, 0), (Corner::UFL, 0), (Corner::DRB, 0), (Corner::DBL, 0),
];
const TC_UP: CornerList = [
	(Corner::UBR, 0), (Corner::ULB, 0), (Corner::DLF, 0), (Corner::DFR, 0),
	(Corner::UFL, 0), (Corner::URF, 0), (Corner::DRB, 0), (Corner::DBL, 0),
];
const TC_DOWN: CornerList = [
	(Corner::URF, 0), (Corner::UBR, 0), (Corner::DBL, 0), (Corner::DLF, 0),
	(Corner::ULB, 0), (Corner::UFL, 0), (Corner::DFR, 0), (Corner::DRB, 0),
];
const TC_BACK: CornerList = [
	(Corner::URF, 0), (Corner::DRB, 1), (Corner::DLF, 0), (Corner::DFR, 0),
	(Corner::UBR, 2), (Corner::UFL, 0), (Corner::DBL, 2), (Corner::ULB, 1),
];
const TC_FRONT: CornerList = [
	(Corner::UFL, 2), (Corner::UBR, 0), (Corner::DFR, 2), (Corner::URF, 1),
	(Corner::ULB, 0), (Corner::DLF, 1), (Corner::DRB, 0), (Corner::DBL, 0),
];
const TC_LEFT: CornerList = [
	(Corner::URF, 0), (Corner::UBR, 0), (Corner::UFL, 1), (Corner::DFR, 0),
	(Corner::DBL, 1), (Corner::ULB, 2), (Corner::DRB, 0), (Corner::DLF, 2),
];
const TC_RIGHT: CornerList = [
	(Corner::DFR, 1), (Corner::URF, 2), (Corner::DLF, 0), (Corner::DRB, 2),
	(Corner::ULB, 0), (Corner::UFL, 0), (Corner::UBR, 1), (Corner::DBL, 0),
];

const fn chain_corners(t1: CornerList, t2: CornerList) -> CornerList {
	let mut out = TC_BASE;

	const_for!(i in 0..NUM_CORNERS => {
		let (c2, o2) = t2[i];
		let (c1, o1) = t1[c2 as usize];

		let r_ori = if o1 < 3 && o2 < 3 {
			((o1+o2) % 3) as isize
		} else { // LR-Plane Symmetry
			if o1 >= 3 && o2 >= 3 {
				let ori = o1 as isize - o2 as isize;
				if ori < 0 { ori+3 }
				else { ori }
			} else if o2 >= 3 {
				let ori = o1+o2;
				if ori >= 6 { (ori-3) as isize }
				else { ori as isize }
			} else {
				let ori = o1 as isize - o2 as isize;
				if ori < 0 { ori+3 }
				else { ori }
			}
		} as Ori;

		out[i] = (c1, r_ori);
	});

	out
}

const fn generate_corner_transform_table() -> [[CornerList; NUM_TURNWISES]; NUM_TURNTYPES] {
	const BASE: [CornerList; NUM_SIDES] = [
		TC_UP, TC_DOWN,
		TC_BACK, TC_FRONT,
		TC_LEFT, TC_RIGHT,
	];

	let mut out = [[TC_BASE; NUM_TURNWISES]; NUM_TURNTYPES];

	const_for!(i in 0..NUM_SIDES => {
		out[i][0] = BASE[i];
		out[i][1] = chain_corners(out[i][0], out[i][0]);
		out[i][2] = chain_corners(out[i][0], out[i][1]);
	});

	out
}

const CORNER_TRANSFORM: [[CornerList; NUM_TURNWISES]; NUM_TURNTYPES] = generate_corner_transform_table();

// ===== Edge Transformations =====
const TE_BASE: EdgeList = [
	(Edge::UF,0), (Edge::UR,0), (Edge::UB,0), (Edge::UL,0),
	(Edge::DF,0), (Edge::DR,0), (Edge::DB,0), (Edge::DL,0),
	(Edge::FR,0), (Edge::BR,0), (Edge::BL,0), (Edge::FL,0),
];
const TE_UP: EdgeList = [
	(Edge::UR,0), (Edge::UB,0), (Edge::UL,0), (Edge::UF,0),
	(Edge::DF,0), (Edge::DR,0), (Edge::DB,0), (Edge::DL,0),
	(Edge::FR,0), (Edge::BR,0), (Edge::BL,0), (Edge::FL,0),
];
const TE_DOWN: EdgeList = [
	(Edge::UF,0), (Edge::UR,0), (Edge::UB,0), (Edge::UL,0),
	(Edge::DL,0), (Edge::DF,0), (Edge::DR,0), (Edge::DB,0),
	(Edge::FR,0), (Edge::BR,0), (Edge::BL,0), (Edge::FL,0),
];
const TE_BACK: EdgeList = [
	(Edge::UF,0), (Edge::UR,0), (Edge::BR,1), (Edge::UL,0),
	(Edge::DF,0), (Edge::DR,0), (Edge::BL,1), (Edge::DL,0),
	(Edge::FR,0), (Edge::DB,1), (Edge::UB,1), (Edge::FL,0),
];
const TE_FRONT: EdgeList = [
	(Edge::FL,1), (Edge::UR,0), (Edge::UB,0), (Edge::UL,0),
	(Edge::FR,1), (Edge::DR,0), (Edge::DB,0), (Edge::DL,0),
	(Edge::UF,1), (Edge::BR,0), (Edge::BL,0), (Edge::DF,1),
];
const TE_LEFT: EdgeList = [
	(Edge::UF,0), (Edge::UR,0), (Edge::UB,0), (Edge::BL,0),
	(Edge::DF,0), (Edge::DR,0), (Edge::DB,0), (Edge::FL,0),
	(Edge::FR,0), (Edge::BR,0), (Edge::DL,0), (Edge::UL,0),
];
const TE_RIGHT: EdgeList = [
	(Edge::UF,0), (Edge::FR,0), (Edge::UB,0), (Edge::UL,0),
	(Edge::DF,0), (Edge::BR,0), (Edge::DB,0), (Edge::DL,0),
	(Edge::DR,0), (Edge::UR,0), (Edge::BL,0), (Edge::FL,0),
];

const fn chain_edges(t1: EdgeList, t2: EdgeList) -> EdgeList {
	let mut out = TE_BASE;

	const_for!(i in 0..NUM_EDGES => {
		let (e2, o2) = t2[i];
		let (e1, o1) = t1[e2 as usize];

		out[i] = (e1, (o1+o2)&1);
	});

	out
}

const fn generate_edge_transform_table() -> [[EdgeList; NUM_TURNWISES]; NUM_TURNTYPES] {
	const BASE: [EdgeList; NUM_SIDES] = [
		TE_UP, TE_DOWN,
		TE_BACK, TE_FRONT,
		TE_LEFT, TE_RIGHT,
	];

	let mut out = [[TE_BASE; NUM_TURNWISES]; NUM_TURNTYPES];

	const_for!(i in 0..NUM_SIDES => {
		out[i][0] = BASE[i];
		out[i][1] = chain_edges(out[i][0], out[i][0]);
		out[i][2] = chain_edges(out[i][0], out[i][1]);
	});

	out
}

const EDGE_TRANSFORM: [[EdgeList; NUM_TURNWISES]; NUM_TURNTYPES] = generate_edge_transform_table();

// ===== Symmetry Transformations =====

const TC_S_URF3: CornerList = [
	(Corner::URF, 2), (Corner::UFL, 1), (Corner::DRB, 2), (Corner::UBR, 1),
	(Corner::DLF, 2), (Corner::DFR, 1), (Corner::ULB, 2), (Corner::DBL, 1)
];
const TC_S_F2: CornerList = [
	(Corner::DLF, 0), (Corner::DBL, 0), (Corner::URF, 0), (Corner::UFL, 0),
	(Corner::DRB, 0), (Corner::DFR, 0), (Corner::ULB, 0), (Corner::UBR, 0)
];
const TC_S_U4: CornerList = [
	(Corner::UBR, 0), (Corner::ULB, 0), (Corner::DFR, 0), (Corner::DRB, 0),
	(Corner::UFL, 0), (Corner::URF, 0), (Corner::DBL, 0), (Corner::DLF, 0)
];
const TC_S_LR: CornerList = [
	(Corner::UFL, 3), (Corner::ULB, 3), (Corner::DFR, 3), (Corner::DLF, 3),
	(Corner::UBR, 3), (Corner::URF, 3), (Corner::DBL, 3), (Corner::DRB, 3)
];


const TE_S_URF3: EdgeList = [
	(Edge::FR, 0), (Edge::UF, 1), (Edge::FL, 0), (Edge::DF, 1),
	(Edge::BR, 0), (Edge::UB, 1), (Edge::BL, 0), (Edge::DB, 1),
	(Edge::UR, 1), (Edge::UL, 1), (Edge::DL, 1), (Edge::DR, 1),
];
const TE_S_F2: EdgeList = [
	(Edge::DF, 0), (Edge::DL, 0), (Edge::DB, 0), (Edge::DR, 0),
	(Edge::UF, 0), (Edge::UL, 0), (Edge::UB, 0), (Edge::UR, 0),
	(Edge::FL, 0), (Edge::BL, 0), (Edge::BR, 0), (Edge::FR, 0),
];
const TE_S_U4: EdgeList = [
	(Edge::UR, 0), (Edge::UB, 0), (Edge::UL, 0), (Edge::UF, 0),
	(Edge::DR, 0), (Edge::DB, 0), (Edge::DL, 0), (Edge::DF, 0),
	(Edge::BR, 1), (Edge::BL, 1), (Edge::FL, 1), (Edge::FR, 1),
];
const TE_S_LR: EdgeList = [
	(Edge::UF, 0), (Edge::UL, 0), (Edge::UB, 0), (Edge::UR, 0),
	(Edge::DF, 0), (Edge::DL, 0), (Edge::DB, 0), (Edge::DR, 0),
	(Edge::FL, 0), (Edge::BL, 0), (Edge::BR, 0), (Edge::FR, 0),
];

pub const NUM_SYMMETRIES: usize = 48;

pub const fn generate_symmetries() -> [(CornerList, EdgeList); NUM_SYMMETRIES] {
	let mut out = [(TC_BASE, TE_BASE); NUM_SYMMETRIES];

	const_for!(x1 in 0..3 => {
		const_for!(x2 in 0..2 => {
			const_for!(x3 in 0..4 => {
				const_for!(x4 in 0..2 => {
					let mut tc = TC_BASE;
					let mut te = TE_BASE;
					const_for!(_ in 0..x1 => {
						tc = chain_corners(tc, TC_S_URF3);
						te = chain_edges(te, TE_S_URF3);
					});
					const_for!(_ in 0..x2 => {
						tc = chain_corners(tc, TC_S_F2);
						te = chain_edges(te, TE_S_F2);
					});
					const_for!(_ in 0..x3 => {
						tc = chain_corners(tc, TC_S_U4);
						te = chain_edges(te, TE_S_U4);
					});
					const_for!(_ in 0..x4 => {
						tc = chain_corners(tc, TC_S_LR);
						te = chain_edges(te, TE_S_LR);
					});

					let idx = 16*x1 + 8*x2 + 2*x3 + x4;
					out[idx] = (tc,te);
				});
			});

		});
	});

	out
}

pub const SYMMETRIES: [(CornerList, EdgeList); NUM_SYMMETRIES] = generate_symmetries();

const fn is_c_base(c: CornerList) -> bool {
	const_for!(i in 0..NUM_CORNERS => {
		let (e,o) = c[i];
		let (a,b) = TC_BASE[i];
		if e as usize != a as usize || o != b { return false; }
	});
	true
}

const fn is_e_base(c: EdgeList) -> bool {
	const_for!(i in 0..NUM_CORNERS => {
		let (e,o) = c[i];
		let (a,b) = TE_BASE[i];
		if e as usize != a as usize || o != b { return false; }
	});
	true
}

pub const fn generate_symmetry_inverse_list() -> [usize; NUM_SYMMETRIES] {
	let mut out = [NUM_SYMMETRIES; NUM_SYMMETRIES];

	const_for!(i in 0..NUM_SYMMETRIES => {
		const_for!(j in 0.. NUM_SYMMETRIES => {
			let (tc1, te1) = SYMMETRIES[i];
			let (tc2, te2) = SYMMETRIES[j];

			let r1 = is_c_base( chain_corners(tc1, tc2) );
			let r2 = is_e_base( chain_edges(te1, te2) );

			if r1 && r2 {
				out[i] = j;
				break;
			}
		});
	});

	out
}

const SYMMETRY_INVERSE: [usize; NUM_SYMMETRIES] = generate_symmetry_inverse_list();

pub fn get_symmetry(cube: &CubieCube, sym: usize) -> CubieCube {
	let inv = SYMMETRY_INVERSE[sym];
	let c = cube.corners;
	let e = cube.edges;

	let (tc, te) = SYMMETRIES[sym];
	let (tci, tei) = SYMMETRIES[inv];

	let c_res = chain_corners( tc, chain_corners(c, tci));
	let e_res = chain_edges( te, chain_edges(e, tei));

	CubieCube {
		corners: c_res,
		edges: e_res,
	}
}

pub fn get_symmetry_inv(cube: &CubieCube, sym: usize) -> CubieCube {
	get_symmetry(cube, SYMMETRY_INVERSE[sym])
}

// ==========

/// Generate a power list where v[i] = base^i from 0..N (exclusive)
const fn pow_list<const N: usize>(base: usize) -> [usize; N] {
	let mut out = [1; N];
	const_for!(i in 1..N => { out[i] = out[i-1] * base; });
	out
}

impl CubieCube {
	pub const fn new() -> Self {
		CubieCube {
			corners: TC_BASE,
			edges: TE_BASE,
		}
	}

	/// Get the corner and orientation at c
	pub const fn corner(&self, c: Corner) -> (Corner, Ori) {
		self.corners[c as usize]
	}

	/// Get the edge and orientation at e
	pub const fn edge(&self, e: Edge) -> (Edge, Ori) {
		self.edges[e as usize]
	}

	/// Return the cube's corner orientation coordinate
	pub fn get_corner_orientation_coord(&self) -> usize {
		const POW: [usize; NUM_CORNERS] = pow_list::<NUM_CORNERS>(3);

		let mut x = 0;
		for corner in Corner::iter() {
			let (_c, o) = self.corner(corner);
			if 6 < corner as usize { continue; }
			x += o as usize * POW[corner as usize];
		}

		x
	}

	/// Return the cube's edges orientation coordinate
	pub fn get_edge_orientation_coord(&self) -> usize {
		const POW: [usize; NUM_EDGES] = pow_list::<NUM_EDGES>(2);

		let mut x = 0;
		for edge in Edge::iter().take(NUM_EDGES-1) {
			let (_e, o) = self.edge(edge);
			x += o as usize * POW[edge as usize];
		}

		x
	}

	/// Return the cube's udslice position coordinate
	/// That's the coordinate which tells us the position of the UD-Slice (FR, BR, BL, FL)
	pub fn get_udslice_coord(&self) -> usize {
		// https://kociemba.org/math/UDSliceCoord.htm
		const UDSLICE: [Edge; 4] = [Edge::FR, Edge::BR, Edge::BL, Edge::FL];
		let chosen = Edge::iter().map(|pos| {
			let (e,_) = self.edge(pos);
			UDSLICE.contains(&e)
		}).collect();

		map_nck(&chosen)
	}

	/// Return the cube's corner permutation as a coordinate
	pub fn get_corner_perm_coord(&self) -> usize {
		let perm = self.corners.iter().map(|(c,_)| *c as usize).collect();
		map_permutation(&perm)
	}

	/// Return the cube's edge permutation as a coordinate
	pub fn get_edge_perm_coord(&self) -> usize {
		let perm = self.edges.iter().take(8).map(|(e,_)| *e as usize).collect();
		map_permutation(&perm)
	}

	pub fn apply_transformation(&mut self, tc: CornerList, te: EdgeList) {
		self.corners = chain_corners(self.corners, tc);
		self.edges = chain_edges(self.edges, te);
	}

	pub fn is_solved(&self) -> bool {
		self.edges == TE_BASE && self.corners == TC_BASE
	}
}

impl RubiksCube for CubieCube {
    fn apply_turn(&mut self, turn: Turn) {
		let tc = CORNER_TRANSFORM[turn.side as usize][turn.wise as usize];
		let te = EDGE_TRANSFORM[turn.side as usize][turn.wise as usize];
		self.apply_transformation(tc, te);
	}
}

impl From<arraycube::ArrayCube> for CubieCube {
	fn from(cube: arraycube::ArrayCube) -> Self {
		let mut out = CubieCube::new();

		for edge in Edge::iter() {
			let (e,o) = cube.get_edge_at_pos(edge).unwrap();
			out.edges[ edge as usize ] = (e, o as Ori);
		}

		for corner in Corner::iter() {
			let (c,o) = cube.get_corner_at_pos(corner).unwrap();
			out.corners[ corner as usize ] = (c, o as Ori);
		}

		out
	}
}

impl Into<arraycube::ArrayCube> for CubieCube {
	fn into(self) -> arraycube::ArrayCube {
		let mut out = arraycube::ArrayCube::new();

		for pos in Corner::iter() {
			let (c, o) = self.corner(pos);

			// The 3 indices to write to
			let (i1,i2,i3) = corner_to_indices(pos);
			// The actual 3 colors there
			let (c1,c2,c3) = corner_to_indices(c);
			let cols = [c1,c2,c3];

			out.data[i1] = cols[o as usize % 3] as u8;
			out.data[i2] = cols[(1+o) as usize % 3] as u8;
			out.data[i3] = cols[(2+o) as usize % 3] as u8;
		}

		for pos in Edge::iter() {
			let (e, o) = self.edge(pos);

			// The 2 indices to write to
			let (i1,i2) = edge_to_indices(pos);
			// The actual 2 colors there
			let (c1,c2) = edge_to_indices(e);
			let cols = [c1,c2];

			out.data[i1] = cols[o as usize % 2] as u8;
			out.data[i2] = cols[(1+o) as usize % 2] as u8;
		}

		out
	}
}

#[cfg(test)]
mod tests {
    use arraycube::ArrayCube;

    use super::*;

	#[test]
	/// Check that all basic turnings result to neutral after 4 turns
	fn cubiecube_turns1() {
		let mut cube = CubieCube::new();

		let turns = parse_turns("U D B F L R").unwrap();

		for turn in turns {
			for _ in 0..4 { cube.apply_turn(turn); }

			if cube != CubieCube::new() {
				panic!("Turn {} doesn't result to neutral element after 4 turns.", turn);
			}
		}
	}

	#[test]
	/// Check that the conversion between CubieCube to ArrayCube seems to work
	fn arraycube_cubiecube_conversion() {
		let turns = parse_turns("L B R2 U D' R D2 L U' R' B2").unwrap();

		let mut array = ArrayCube::new();
		let mut cubie = CubieCube::new();

		for turn in turns {
			array.apply_turn(turn);
			cubie.apply_turn(turn);

			let acubie: ArrayCube = cubie.clone().into();
			let carray: CubieCube = array.clone().into();
			assert!(acubie == array);
			assert!(carray == cubie);
		}
	}

	#[test]
	/// Check that symmetries are the same in arraycube and cubiecube
	fn array_cubiecube_symmetries() {
		let turns = parse_turns("L B R2 U D' R D2 L U' R' B2").unwrap();

		let mut array = ArrayCube::new();
		let mut cubie = CubieCube::new();

		for turn in turns {
			array.apply_turn(turn);
			cubie.apply_turn(turn);

			for i in 0..NUM_SYMMETRIES {
				let sarray = arraycube::get_symmetry(&array, i);
				let scubie = cubiecube::get_symmetry(&cubie, i);

				let convert: ArrayCube = scubie.into();

				if convert != sarray {
					println!("How it should be:");
					sarray.print();
					println!("What it is:");
					convert.print();
					panic!("Symmetry number {} differ!", i);
				}
			}
		}
	}
}