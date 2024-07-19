use const_for::const_for;

/// Generate a faculty list where v[i] = (i!) until N (exclusive)
const fn fac_list<const N: usize>() -> [usize; N] {
	let mut out = [1; N];
	const_for!(i in 1..N => { out[i] *= out[i-1] * i; });
	out
}

/// Returns n choose k (the number of ways choosing k elements from n)
const fn n_choose_k(n: usize, k: usize) -> usize {
	FAC[n] / (FAC[n-k] * FAC[k])
}

// 12! fits into 32bit integers
const FAC: [usize; 12] = fac_list::<12>();

/// Calculates (base^exp) in O(log n)
pub const fn fast_pow(base: usize, exp: usize) -> usize {
	let mut exp = exp;
	let mut b = 1;
	let mut x = 1;

	while exp != 0 {
		if (exp&1) == 1 { x *= b; }
		b *= base;
		exp >>= 1;
	}

	x
}

/// Map a permutation to a number
/// The permutation MUST only containt values from 0..n (n exclusive)
/// where n is the lenght of PERM.
///
/// Runtime: O(n log n)
/// Additional Memory: O(n)
pub fn map_permutation(perm: &Vec<usize>) -> usize {
	let n = perm.len();

	// Fenwicktree
	let mut fenw = vec![0usize; n];

	let mut x = 0;
	for (i, ele) in perm.iter().enumerate() {
		let mut cnt = fenw[0];

		let mut v = *ele as isize;
		// Prefix sum until { perm[i] }
		while v != 0 {
			cnt += fenw[v as usize];
			v -= v & -v;
		}

		// Update { perm[i] }
		if *ele == 0 {
			fenw[0] += 1;
		} else {
			v = *ele as isize;
			while (v as usize) < n {
				fenw[v as usize] += 1;
				v += v & -v;
			}
		}

		// i-cnt = The number of indices j where perm[j] > perm[i]
		x += FAC[i]*(i-cnt);
	}

	x
}

/// Return the k-th permutation of size n.
/// (Inverse of 'map_permutation')
///
/// Runtime: O(N^2)
/// Additional Memory O(n)
pub fn get_kth_perm(n: usize, k: usize) -> Vec<usize> {
	let mut out = vec![0; n];
	let mut perm: Vec<_> = (0..n).collect();
	let mut k = k;

	for i in (0..n).rev() {
		let pos = k / FAC[i];
		out[i] = perm[i-pos];
		k %= FAC[i];
		perm.remove(i-pos); // this makes it slow
	}

	out
}

/// Maps a given "n choose k" to a number.
/// v[i] = true if the i-th element was chosen
/// n, k is automatically determined by v
///
/// Runtime: O(n)
/// Additional Memory O(1)
pub fn map_nck(v: &Vec<bool>) -> usize {
	let mut x = 0;
	let mut k: usize = 0;

	for (n, chosen) in v.iter().enumerate() {
		if *chosen {
			k += 1;
		} else if k > 0 {
			x += n_choose_k(n, k-1);
		}
	}

	x
}

/// Return a the i-th version of "n choose k".
/// (Inverse of 'map_cnk')
/// If n < k, or i >= n choose k the output is meaningless.
///
/// Runtime O(n)
/// Additional memory O(1)
pub fn get_nck(n: usize, k: usize, i: usize) -> Vec<bool> {
	let mut out = vec![false; n];

	if k == 0 { return out; }

	let mut x = i;
	let mut k = k-1;

	for i in (0..n).rev() {
		let nck = n_choose_k(i, k);

		if x >= nck {
			x -= nck;
		} else {
			out[i] = true;
			if k == 0 { break; }
			k -= 1;
		}
	}

	out
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	/// Test that the mapping of the i-th permutation is correct
	fn permutation_mapping() {
		for i in 0..9 {
			for j in 0..FAC[i] {
				let v = get_kth_perm(i, j);
				let idx = map_permutation(&v);

				if idx != j {
					panic!("Permutation {:?} (generated from {}) is mapped to {}!", v, j, idx);
				}
			}
		}
	}

	#[test]
	/// Test that the mapping of "n chhose k" vectors are correct
	fn nck_mapping() {
		for i in 0..6 {
			for j in 0..=i {
				let ways = n_choose_k(i,j);
				for k in 0..ways {
					let nck = get_nck(i, j, k);
					let idx = map_nck(&nck);

					if k != idx {
						panic!("{} choose {}, {}-th version ouputs {:?}, but is mapped to {}", i,j,k, nck, idx);
					}
				}
			}
		}
	}
}
