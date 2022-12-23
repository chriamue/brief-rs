#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use image::GrayImage;
use rand::prelude::*;

// seed
const SEED: u64 = 0x34985739;

/// Define a struct to represent the BRIEF descriptor
pub struct BriefDescriptor {
    test_pairs: Vec<((u32, u32), (u32, u32))>,
}

impl BriefDescriptor {
    /// Implement the "new" method to create a new BRIEF descriptor
    pub fn new(num_test_pairs: usize, cell_side: u32) -> Self {
        // Generate a set of random test pairs
        let mut rng = StdRng::seed_from_u64(SEED);
        let test_pairs: Vec<((u32, u32), (u32, u32))> = (0..num_test_pairs)
            .map(|_| {
                (
                    (rng.gen_range(0..cell_side), rng.gen_range(0..cell_side)),
                    (rng.gen_range(0..cell_side), rng.gen_range(0..cell_side)),
                )
            })
            .collect();

        Self { test_pairs }
    }

    /// Implement a method to compute the BRIEF descriptor for a patch of an image
    pub fn compute(&self, patch: &GrayImage) -> Vec<u8> {
        let mut descriptor = Vec::new();

        // For each test pair, calculate the difference between the pixel values
        // and encode the result as a binary value
        for (i, j) in &self.test_pairs {
            let diff: i32 = patch[*i].0[0] as i32 - patch[*j].0[0] as i32;
            descriptor.push(if diff > 0 { 1 } else { 0 });
        }

        descriptor
    }
}

#[cfg(test)]
mod tests {
    use image::GenericImage;

    use super::*;

    #[test]
    fn it_works() {
        // Create a new BRIEF descriptor with 32 test pairs
        let brief = BriefDescriptor::new(32, 32);

        // Load an image and extract a patch
        let mut img = image::open("Lenna_gray.jpg").unwrap().into_luma8();
        let patch = &img.sub_image(100, 100, 50, 50).to_image();

        // Compute the BRIEF descriptor for the patch
        let descriptor = brief.compute(patch);
        println!("{:?}", descriptor);
    }
}
