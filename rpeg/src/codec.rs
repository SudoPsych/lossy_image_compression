use csc411_image::{Read, Write, RgbImage};
use csc411_arith::{chroma_of_index, index_of_chroma};
use bitpack;
use csc411_rpegio::{read_in_rpeg_data, output_rpeg_data};
use crate::dct_trans;

pub fn compress(filename: Option<&str>) {
	
	let image = RgbImage::read(filename).unwrap();
	let mut words: Vec<[u8; 4]> = Vec::new();
	// set last bit to 0, rounds down to nearest even number
	let width = image.width & !1_u32;
	let height = image.height & !1_u32;
	let num_blocks = (width * height) / 4;
	
	for num in 0..num_blocks {
		let pixel_block = get_pixel_data(&image, num);
		let ypbpr_data: Vec<(f32, f32, f32)> = pixel_block.into_iter().map(|(r, g, b)| dct_trans::rgb_to_ypbpr(r, g, b)).collect();
		
		let mut y_vec: Vec<f32> = Vec::new();
		let mut pb_avg: f32 = 0.0;
		let mut pr_avg: f32 = 0.0;
		
		for (y, pb, pr) in ypbpr_data {
			y_vec.push(y);
			pb_avg += pb;
			pr_avg += pr;
		}
		pb_avg /= 4.0;
		pr_avg /= 4.0;
		
		let (a, b, c, d) = dct_trans::discrete_cosine_transformation(y_vec[0], y_vec[1], y_vec[2], y_vec[3]);
		
		// q_ => quantized_
		let q_a = dct_trans::quantize(a, true) as u64;
		let q_b = dct_trans::quantize(b, false);
		let q_c = dct_trans::quantize(c, false);
		let q_d = dct_trans::quantize(d, false);
		
		let pb_index = csc411_arith::index_of_chroma(pb_avg) as u64;
		let pr_index = csc411_arith::index_of_chroma(pr_avg) as u64;
		
		let word = pack(q_a, q_b, q_c, q_d, pb_index, pr_index) as u32;
		words.push(word.to_be_bytes());
	}
	output_rpeg_data(&words, width, height);
}

pub fn decompress(filename: Option<&str>) {
	
	let (raw_bytes, width, height) = csc411_rpegio::read_in_rpeg_data(filename);
	
	let num_blocks = (width / 2) * (height / 2);
	let mut new_image = csc411_image::RgbImage {
		pixels: vec![csc411_image::Rgb{ red: 255, green: 255, blue: 255 }; (width * height) as usize],
		width,
		height,
		denominator: 255
	};
	for num in 0..num_blocks {
		// top left corner of block
		let tlc = ((num / (width / 2)) * (width * 2)) + ((num % (width / 2)) * 2);
		
		// top left, top right, bottom left, bottom right
		let p1_index = (tlc) as usize;
		let p2_index = (tlc + 1) as usize;
		let p3_index = (tlc + width) as usize;
		let p4_index = (tlc + width + 1) as usize;
		let pixel_indeces = vec![p1_index, p2_index, p3_index, p4_index];

		let word = u32::from_be_bytes(raw_bytes[num as usize]) as u64;
		let (q_a, q_b, q_c, q_d, pb_index, pr_index) = unpack(word);
		
		let a = dct_trans::expand(q_a as f32, true);
		let b = dct_trans::expand(q_b as f32, false);
		let c = dct_trans::expand(q_c as f32, false);
		let d = dct_trans::expand(q_d as f32, false);
		
		let (y1, y2, y3, y4) = dct_trans::inverse_discrete_cosine_transformation(a, b, c, d);
		
		let pr = csc411_arith::chroma_of_index(pr_index as usize);
		let pb = csc411_arith::chroma_of_index(pb_index as usize);
		
		let rgb_vec: Vec<(f32, f32, f32)> = vec![y1, y2, y3, y4].into_iter().map(|y| dct_trans::ypbpr_to_rgb(y, pb, pr)).collect();
		
		for (index, rgb_values) in pixel_indeces.into_iter().zip(rgb_vec.into_iter()) {
			set_pixel_data(&mut new_image.pixels[index], rgb_values);
		}
	}
	new_image.write(None);
}

fn set_pixel_data(pixel: &mut csc411_image::Rgb, rgb_values: (f32, f32, f32)) {
	
	pixel.red   = (pixel.red   as f32 * rgb_values.0).round() as u16;
	pixel.green = (pixel.green as f32 * rgb_values.1).round() as u16;
	pixel.blue  = (pixel.blue  as f32 * rgb_values.2).round() as u16;
	
}

fn get_pixel_data(image: &csc411_image::RgbImage, num: u32) -> Vec<(f32, f32, f32)> {
	let w = image.width;
	let tlc = ((num / (w / 2)) * (w * 2)) + ((num % (w / 2)) * 2);
	
	// top left, top right, bottom left, bottom right
	let p1 = &image.pixels[(tlc) as usize];
	let p2 = &image.pixels[(tlc + 1) as usize];
	let p3 = &image.pixels[(tlc + w) as usize];
	let p4 = &image.pixels[(tlc + w + 1) as usize];

	let pixel_vec = vec![p1, p2, p3, p4];
	
	pixel_vec.iter().map(|&pixel| (pixel.red   as f32 / image.denominator as f32,
								   pixel.green as f32 / image.denominator as f32,
								   pixel.blue  as f32 / image.denominator as f32)
								  ).collect()
	
}

fn unpack(word: u64) -> (u64, i64, i64, i64, u64, u64) {
	
	let a = bitpack::bitpack::getu(word, 9, 23);
	let b = bitpack::bitpack::geti(word, 5, 18);
	let c = bitpack::bitpack::geti(word, 5, 13);
	let d = bitpack::bitpack::geti(word, 5,  8);
	let pb = bitpack::bitpack::getu(word, 4, 4);
	let pr = bitpack::bitpack::getu(word, 4, 0);

	(a, b, c, d, pb, pr)
}

fn pack(a: u64, b: i64, c: i64, d: i64, pb: u64, pr: u64) -> u64 {
	
	let mut word = 0_u64;
	
	word = bitpack::bitpack::newu(word, 9, 23, a).unwrap();
	word = bitpack::bitpack::newi(word, 5, 18, b).unwrap();
	word = bitpack::bitpack::newi(word, 5, 13, c).unwrap();
	word = bitpack::bitpack::newi(word, 5, 8,  d).unwrap();
	word = bitpack::bitpack::newu(word, 4, 4, pb).unwrap();
	word = bitpack::bitpack::newu(word, 4, 0, pr).unwrap();
	
	word
}

