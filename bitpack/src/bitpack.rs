
pub fn fitsi(n: i64, width: u64) -> bool {
    n == ((n << (64 - width)) as i64) >> (64 - width)
}

pub fn fitsu(n: u64, width: u64) -> bool {
    n == (n << (64 - width)) >> (64 - width)
}

pub fn geti(word: u64, width: u64, lsb: u64) -> i64 {
	// shift relevant bits to be left aligned
	// shift to be right aligned (fills left side with 1's if negative, 0's if positive)
	// return (0x value if positive) (1x value if negative)
    ((word << (64 - width - lsb)) as i64) >> (64 - width)
}

pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
	// shift relevant bits to be left aligned (deletes bits we don't want on the left)
	// shift relevant bits to be right aligned (deletes bits to the right)
	// return 0x value
    (word << (64 - width - lsb)) >> (64 - width)
}

pub fn newi(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitsi(value, width) {
    	// -1_i128 for the edge case of width 64, couldn't find a better way to do that
    	Some((!((!0_u64 >> (64 - width)) << lsb) & word) | (((value & (!(-1_i128 << width)) as i64) as u64) << lsb))
    } else {
    	None
    }
    // & value bits with (0x width 1's) to extract only relevant values
	// shift value bits to where it will be placed into word
	// | value bits into word
	// 0xffffffffffffffff >> (64 - width)
}

pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if fitsu(value, width) {
    	Some((!((!0_u64 >> (64 - width)) << lsb) & word) | (value << lsb))
    } else {
    	None
    }
}

#[cfg(test)]
mod tests {
	use crate::bitpack::*;
	// 8 bit width tests
	
	// fitsi
	// n bounded by {-128, 127}
	#[test]
	fn fitsi_lower_bound() {
		assert!(fitsi(-128, 8));
		assert!(!fitsi(-129, 8));
	}
	// n bounded by {0, 255}
	#[test]
	fn fitsi_upper_bound() {
		assert!(fitsi(127, 8));
		assert!(!fitsi(128, 8));
	}
	
	//fitsu
	// n bounded by {0, 255}
	#[test]
	fn fitsu_lower_wound() {
		assert!(fitsu(0, 8));
	}
	#[test]
	fn fitsu_upper_bound() {
		assert!(fitsu(255, 8));
		assert!(!fitsu(256, 8));
	}
	
	// build_word and get_word are general tests
	#[test]
	fn build_word() {
		// original word
		// 0 x 32 ... 1 x 32
		// numbers to input (left to right)
		// -3 4 1 15 2 6 -8 -1
		// result
		// 0xD41F286
		let mut word: u64 = !0_u32 as u64;
		// newi and newu should delete the 1's in their way
		word = newi(word, 4, 28, -3).unwrap();
		word = newu(word, 4, 24, 4 ).unwrap();
		word = newu(word, 4, 20, 1 ).unwrap();
		word = newu(word, 4, 16, 15).unwrap();
		word = newu(word, 4, 12, 2 ).unwrap();
		word = newu(word, 4, 8, 6  ).unwrap();
		word = newi(word, 4, 4, -8 ).unwrap();
		word = newi(word, 4, 0, -1 ).unwrap();
		
		assert_eq!(word, 0xD41F268F);
	}
	
	#[test]
	fn get_word() {
		let word: u64 = 0xD41F268F;
		assert_eq!(geti(word, 4, 28), -3);
		assert_eq!(getu(word, 4, 24),  4);
		assert_eq!(getu(word, 4, 20),  1);
		assert_eq!(getu(word, 4, 16), 15);
		assert_eq!(getu(word, 4, 12),  2);
		assert_eq!(getu(word, 4,  8),  6);
		assert_eq!(geti(word, 4,  4), -8);
		assert_eq!(geti(word, 4,  0), -1);
	}
	
	// follwing tests check if user gives maximum width of 64
	#[test]
	fn newu_bounds() {
		let mut word: u64 = 0;
		word = newu(word, 64, 0, !0_u64).unwrap();
		assert_eq!(word, !0_u64);
	}
	#[test]
	fn newi_bounds() {
		let mut word: u64 = 0;
		word = newi(word, 64, 0, -1_i64).unwrap();
		assert_eq!(word, !0_u64);
	}
	
	#[test]
	fn getu_bounds() {
		let word: u64 = !0_u64;
		assert_eq!(getu(word, 64, 0), !0_u64);
	}
	#[test]
	fn geti_bounds() {
		let word: u64 = !0_u64;
		assert_eq!(geti(word, 64, 0), -1_i64);
	}
	
}

