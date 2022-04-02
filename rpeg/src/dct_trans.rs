pub fn rgb_to_ypbpr(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
	
	let y  = (0.299 * r) + (0.587 * g) + (0.114 * b);
	let pb = (-0.168736 * r) + (-0.331264 * g) + (0.5 * b);
	let pr = (0.5 * r) + (-0.418688 * g) + (-0.081312 * b);
	
	(y, pb, pr)
}

pub fn ypbpr_to_rgb(y: f32, pb: f32, pr: f32) -> (f32, f32, f32) {
	
	let r = y + (1.402 * pr);
	let g = y + (-0.344136 * pb) + (-0.714136 * pr);
	let b = y + (1.772 * pb);

	(r, g, b)
}

pub fn discrete_cosine_transformation(y1: f32, y2: f32, y3: f32, y4: f32) -> (f32, f32, f32, f32) {
	
	let a = (y4 + y3 + y2 + y1) / 4.0;
	let b = (y4 + y3 - y2 - y1) / 4.0;
	let c = (y4 - y3 + y2 - y1) / 4.0;
	let d = (y4 - y3 - y2 + y1) / 4.0;
	
	(a, b, c, d)
}

pub fn inverse_discrete_cosine_transformation(a: f32, b: f32, c: f32, d: f32) -> (f32, f32, f32, f32) {
	
	let y1 = a - b - c + d;
	let y2 = a - b + c - d;
	let y3 = a + b - c - d;
	let y4 = a + b + c + d;
	
	(y1, y2, y3, y4)
}

pub fn quantize(num: f32, is_a: bool) -> i64 {
	if is_a {
		(num * 511_f32).round() as i64
	} else {
		(num * 50_f32).round().clamp(-15.0, 15.0) as i64
	}
}

pub fn expand(num: f32, is_a: bool) -> f32 {
	if is_a {
		num / 511.0
	} else {
		num / 50.0
	}
}

#[cfg(test)]
mod tests {
	use crate::dct_trans::*;
	#[macro_use]
	use approx::assert_relative_eq;
	
	#[test]
	fn perform_discrete_cosine_transformation_and_invert() {
		// round trip test DCT and IDCT
		// test y values from 0.0 to 1.0
		for x1 in 0..=10 {
			for x2 in 0..=10 {
				for x3 in 0..=10 {
					for x4 in 0..=10 {
						let y1 = x1 as f32 / 10.0;
						let y2 = x2 as f32 / 10.0;
						let y3 = x3 as f32 / 10.0;
						let y4 = x4 as f32 / 10.0;
						
						let (a, b, c, d) = discrete_cosine_transformation(y1, y2, y3, y4);
						let (new_y1, new_y2, new_y3, new_y4) = inverse_discrete_cosine_transformation(a, b, c, d);
						// converted cosine coefficients should be no more than a floating point rounding error away from original values
						assert_relative_eq!(y1, new_y1, epsilon=0.00001);
						assert_relative_eq!(y2, new_y2, epsilon=0.00001);
						assert_relative_eq!(y3, new_y3, epsilon=0.00001);
						assert_relative_eq!(y4, new_y4, epsilon=0.00001);
						
					}
				}
			}
		}
	}
	
	#[test]
	// round trip testing of quantizing and expanding a value
	fn quantize_and_expand() {
		for x in -500..=500 {
			// test from -0.500 to 0.500
			let num = x as f32 / 1000.0;
			let q = quantize(num, false) as f32;
			let e = expand(q, false);
			// original value should never stray more than 0.01 from round trip value (after clamping)
			assert_relative_eq!(num.clamp(-0.3, 0.3), e, epsilon=0.01001);
		}
	}
	
	#[test]
	// round trip testing of converting an rgb values to ypbpr and back
	fn converting_rgb_to_ypbpr_and_back() {
		for r in 0..=255 {
	        for g in 0..=255 {
	            for b in 0..=255 {
					let (y, pb, pr) = rgb_to_ypbpr(r as f32 / 255.0,
												   g as f32 / 255.0,
												   b as f32 / 255.0);
					
					let (new_r, new_g, new_b) = ypbpr_to_rgb(y, pb, pr);
					// converting rgb to ypbpr and back should result in the same original rgb values (after rounding)
					assert_eq!((new_r * 255.0).round(), r as f32);
					assert_eq!((new_g * 255.0).round(), g as f32);
					assert_eq!((new_b * 255.0).round(), b as f32);
	            }
	        }
	    }
	}
}
