extern crate image;
extern crate clap;

use std::fs::File;
use std::path::Path;
use clap::{Arg, App};
use image::GenericImage;

fn rgb_comp<F>(a: &[u8], b: &[u8], cfunc: F) -> bool 
    where F: Fn(&u8, &u8) -> bool {
    // comparision function; takes two u8 slices and compares their values element wise
    // before performing an all fold
    let mut v_bool = Vec::new();
    for (i,j) in a.iter().zip(b.iter()) {
        v_bool.push( cfunc(i,j) );
    }
    v_bool.iter().fold(true, |x, y| x & y)
}


fn main() {
    // setup clap argument parser
    let matches = App::new("rusty-transparent")
                    .version("0.0.1")
                    .author("edelsonc")
                    .about("Simple Image Background Transparency App")
                    .args( &[
                    Arg::from_usage("<INPUT> 'Sets the image file'"),
                    Arg::from_usage("[color], -c, --color [val1] [val2] [val3] 'Sets rgb value to compare'")
                    ])
                    .get_matches();
    
    // get matching values of arguments or substitute defaults
    let image_name = matches.value_of("INPUT").unwrap();
    let rgb_arg = match matches.values_of_lossy("color") {
        Some(v) => v,
        None => vec!["255".to_string(), "255".to_string(), "255".to_string()],
    };
    
    let rgb: Vec<u8> = rgb_arg.iter().map(|s| s.parse::<u8>().unwrap()).collect();
    
    // open image as a new dynamic image object
    let img = match image::open(&Path::new(image_name)) {
        Ok(p) => p,
        Err(e) => panic!("Not a valid image path or could not open image"),
    };

    let mut imgbuf = img.to_rgba();
    
    // iterate through the pixels of the image
    for pixel in imgbuf.pixels_mut() {
        if rgb_comp(&pixel.data[0..3], &rgb[..], |x,y| x == y) {
            pixel[3] = 0;
        }
    }

    // create and save to external file
    let mut fout = File::create(&Path::new("test.png")).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(&mut fout, image::PNG);
}
