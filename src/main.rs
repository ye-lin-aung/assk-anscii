use std::fs;
use std::fs::File;

extern crate image;
use image::GenericImageView;
use image::imageops::FilterType;

fn main() {
    let image = match image::open("assk.jpg"){

        Ok(image) => image,
        Err(error) => panic!("Problem reading image: {:?}", error)
    };
    println!("dimensions {:?}", image.dimensions());

    let gray_image = image.grayscale();
	
    let gray_image = gray_image.resize(gray_image.width() / 5 , gray_image.height() / 5, FilterType::Nearest);


    //let character_set: [&str; 12] = [" ", "'", ",", ".", ":", ";", ".", "L", "O", "0", "#", "@"];
    let character_set : [&str; 12] = ["@", "#", "0" , "O", "L", ".", ";", ":", ".", ",", "'", " "];

    let mut art = String::new();
    let mut last_y = 0;

    for pixel in gray_image.pixels() {
        if last_y != pixel.1 {
            art.push_str("\n");
            last_y = pixel.1;
        }

        let pixel_data = pixel.2;

        let brightness:f64 = ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
        let character_position = ((brightness/255.0) * (character_set.len()  - 1) as f64 ).round() as usize;
        art.push_str(character_set[character_position]);
    }
    fs::write("assk.txt", art.as_bytes()).unwrap();

    print!("{}", art);


}
