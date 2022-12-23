# brief-rs

This project implements the [Binary Robust Independent Elementary Features (BRIEF)](https://docs.opencv.org/4.5.0/dc/d7d/tutorial_py_brief.html) descriptor in Rust. BRIEF is a local image descriptor that is used for tasks such as image matching and object recognition. It is a fast and lightweight alternative to other local feature descriptors such as SIFT (Scale-Invariant Feature Transform) and SURF (Speeded Up Robust Feature).

## Installation

To use Rust BRIEF in your own project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
brief = "0.1.0"
```

## Usage

```rust
use brief::BriefDescriptor;
use image::{GrayImage, GenericImage};

// Create a new BRIEF descriptor with 32 test pairs
let brief = BriefDescriptor::new(32, 32);

// Load an image and extract a patch
let mut img = image::open("Lenna_gray.jpg").unwrap().into_luma8();
let patch = &img.sub_image(100, 100, 50, 50).to_image();

// Compute the BRIEF descriptor for the patch
let descriptor = brief.compute(patch);
println!("{:?}", descriptor);
```

This example assumes that you have the image crate installed, and that you have an image file named image.jpg in the current directory. It creates a new BRIEF descriptor with a specified number of test pairs, and then uses the compute method to calculate the BRIEF descriptor for a patch of an image.

## License

brief-rs is licensed under the MIT license. See [LICENSE](LICENSE) for more information.

## Note

This project is created partly with the help of ChatGPT.
