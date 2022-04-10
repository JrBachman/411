extern crate array2;
extern crate csc411_image;
use std::collections::HashMap;
use csc411_image::Pixel::Gray;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    //Read in file
    if env::args().len() == 2 {
        let s_image = csc411_image::Image::read(Some(&args[1])).unwrap();
        let array_image = verification(s_image);
        check_row(&array_image);
        check_column(&array_image);
        check_block(&array_image);
        std::process::exit(0);
    } else {
        let s_image = csc411_image::Image::read(None).unwrap();
        let array_image = verification(s_image);
        check_row(&array_image);
        check_column(&array_image);
        check_block(&array_image);
        std::process::exit(0);
    }

    //Iterate through each column and row, ensure each does not contain repeat elements
    //Iterate through each block and ensure does not contain repeat elements
    //Will have to do math to get the index of the bounds of the blocks (3x3)
}

///Checks if file meets parameters
fn verification(s_image: csc411_image::Image) -> array2::Array2<csc411_image::Pixel> {
    //Verify file
    //Doesn't have 9 as denominator, exit 1
    if s_image.denominator != 9 {
        std::process::exit(1);
    }
    //height and width both must be 9, otherwise exit 1
    if s_image.height != 9 || s_image.width != 9 {
        std::process::exit(1);
    }
    //Check that first pixel is valid, and therefore file is valid graymap
    let pixel = &s_image.pixels[0];
    match pixel{
        csc411_image::Pixel::Rgb(_) => std::process::exit(1),
        csc411_image::Pixel::Gray(_) => (), 
    }
    //All of the pixel intensity values must be >= 1 and <= 9
    let array_image =
        array2::Array2::from_row_major(s_image.pixels, (s_image.width, s_image.height));
    for (_, _, pix) in array_image.iter_row_major() {
        if let Gray(v) = pix {
            // println!("count: {}, pix: {}", count, v.value);
            if v.value < 1 || v.value > 9 {
                std::process::exit(1)
            }
        }
    }
    array_image
}

///Checks each row for repeats
fn check_row(array_image: &array2::Array2<csc411_image::Pixel>) {
    let mut count = 0;
    //Generate map for every 9 values
    let mut map: HashMap<u16, u32> = HashMap::new();
    for (_, _, pix) in array_image.iter_row_major() {
        //Check if repeats
        if let Gray(v) = pix {
            if map.keys().any(|&x| x == v.value) {
                std::process::exit(1);
            }
            map.insert(v.value, 0);
            count += 1;
            //Clear map
            if count % 9 == 0 {
                map = HashMap::new();
            }
        }
    }
}

///Checks each column for repeats
fn check_column(array_image: &array2::Array2<csc411_image::Pixel>) {
    let mut count = 0;
    //Generate map for every 9 values
    let mut map: HashMap<u16, u32> = HashMap::new();
    for (_, _, pix) in array_image.iter_column_major() {
        //Check if repeats
        if let Gray(v) = pix {
            if map.keys().any(|&x| x == v.value) {
                std::process::exit(1);
            }
            map.insert(v.value, 0);
            count += 1;
            //Clear map
            if count % 9 == 0 {
                map = HashMap::new();
            }
        }
    }
}

///Checks each sudoku block for repeats
fn check_block(array_image: &array2::Array2<csc411_image::Pixel>) {
    //Create maps for each block
    let mut map1: HashMap<u16, u32> = HashMap::new();
    let mut map2: HashMap<u16, u32> = HashMap::new();
    let mut map3: HashMap<u16, u32> = HashMap::new();
    let mut map4: HashMap<u16, u32> = HashMap::new();
    let mut map5: HashMap<u16, u32> = HashMap::new();
    let mut map6: HashMap<u16, u32> = HashMap::new();
    let mut map7: HashMap<u16, u32> = HashMap::new();
    let mut map8: HashMap<u16, u32> = HashMap::new();
    let mut map9: HashMap<u16, u32> = HashMap::new();
    for (r, c, pix) in array_image.iter_row_major() {
        //Check row, check column bounds
            //check if exists in hashmap
                //insert into associated hashmap
        //Top left
        if (r == 0 || r == 1 || r == 2) && (c == 0 || c == 1 || c == 2) {
            if let Gray(v) = pix {
                if map1.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map1.insert(v.value, 0);
            }
        }
        //Top middle
        if (r == 0 || r == 1 || r == 2) && (c == 3 || c == 4 || c == 5) {
            if let Gray(v) = pix {
                if map2.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map2.insert(v.value, 0);
            }
        }
        //Top right
        if (r == 0 || r == 1 || r == 2) && (c == 6 || c == 7 || c == 8) {
            if let Gray(v) = pix {
                if map3.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map3.insert(v.value, 0);
            }
        }
        //Middle left
        if (r == 3 || r == 4 || r == 5) && (c == 0 || c == 1 || c == 2) {
            if let Gray(v) = pix {
                if map4.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map4.insert(v.value, 0);
            }
        }
        //Center
        if (r == 3 || r == 4 || r == 5) && (c == 3 || c == 4 || c == 5) {
            if let Gray(v) = pix {
                if map5.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map5.insert(v.value, 0);
            }
        }
        //Middle right
        if (r == 3 || r == 4 || r == 5) && (c == 6 || c == 7 || c == 8) {
            if let Gray(v) = pix {
                if map6.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map6.insert(v.value, 0);
            }
        }
        //Bottom left
        if (r == 6 || r == 7 || r == 8) && (c == 0 || c == 1 || c == 2) {
            if let Gray(v) = pix {
                if map7.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map7.insert(v.value, 0);
            }
        }
        //Bottom middle
        if (r == 6 || r == 7 || r == 8) && (c == 3 || c == 4 || c == 5) {
            if let Gray(v) = pix {
                if map8.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map8.insert(v.value, 0);
            }
        }
        //Bottom right
        if (r == 6 || r == 7 || r == 8) && (c == 6 || c == 7 || c == 8) {
            if let Gray(v) = pix {
                if map9.keys().any(|&x| x == v.value) {
                    std::process::exit(1);
                }
                map9.insert(v.value, 0);
            }
        }
    }
}
