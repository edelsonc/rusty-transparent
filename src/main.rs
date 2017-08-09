extern crate image;
extern crate clap;

use std::fs::File;
use std::path::Path;
use clap::{Arg, App};

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

// operator functions to be returned based on chosen operator
fn eq (x: &u8, y: &u8) -> bool { x == y }
fn gt (x: &u8, y: &u8) -> bool { x > y }
fn lt (x: &u8, y: &u8) -> bool { x < y }

fn comp_func(op: &str) -> fn(&u8, &u8) -> bool {
    // function to select operator functon
    if op == "eq" {
        eq 
    } else if op == "gt" {
        gt
    } else {
        lt
    }
}

fn main() {
    // setup clap argument parser
    let matches = App::new("rusty-transparent")
                    .version("0.0.1")
                    .author("edelsonc")
                    .about("Simple Image Background Transparency App")
                    .args( &[
                    Arg::from_usage("<INPUT> 'Sets the image file'"),
                    Arg::from_usage("[color], -c, --color [val1] [val2] [val3] 'Sets rgb value to compare'"),
                    Arg::from_usage("[operator] -o --operator [op] 'Comparison operator'")
                    .possible_values(&["eq", "lt", "gt"])
                    .default_value("eq")
                    ])
                    .get_matches();
    
    // get matching values of arguments or substitute defaults
    let image_name = matches.value_of("INPUT").unwrap();
    let rgb = match matches.values_of_lossy("color") {
        Some(v) => v.iter().map(|s| s.parse::<u8>().unwrap()).collect(),
        None => vec![255u8, 255u8, 255u8],
    };

    // turn matching operator argument into comparision function
    let comp_op = matches.value_of("operator").unwrap(); // safe with default
    let cfunc = comp_func(&comp_op);

    // open image as a new dynamic image object
    let img = match image::open(&Path::new(image_name)) {
        Ok(p) => p,
        Err(e) => panic!("Not a valid image path or could not open image"),
    };

    let mut imgbuf = img.to_rgba();
    
    // iterate through the pixels of the image
    for pixel in imgbuf.pixels_mut() {
        if rgb_comp(&pixel.data[0..3], &rgb[..], cfunc) {
            pixel[3] = 0;
        }
    }

    // create and save to external file
    // following only safe since image_name is a valid path
    let no_ext = Path::new(image_name)
        .file_stem().unwrap()
        .to_str().unwrap();
    let new_title = format!("transparent_{}.png", no_ext); 
    let mut fout = File::create(&Path::new(&new_title)).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(&mut fout, image::PNG);
}

