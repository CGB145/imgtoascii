use image::{ GenericImageView, ImageReader, Pixel, imageops::FilterType::Nearest};
use colored::{self, Colorize};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <image_path> <width> <height>", args[0]);
        std::process::exit(1);
    }

    let img_path = &args[1];

    let img = match ImageReader::open(img_path){
        Ok(img_r) =>{
            match img_r.decode(){
                Ok(dyn_img) => {
                    dyn_img
                },
                Err(err) => {
                    panic!("{}",err)
                }
            }
        },
        Err(err) => {
            panic!("{}",err)
        }
    };


    let width: u32 = args[2].parse().expect("Width must be a valid u32");
    let height: u32 = args[3].parse().expect("Height must be a valid u32");
    let img_resized = img.resize_exact(width, height,Nearest);

    let rgb: Vec<_>  = img_resized.pixels().map(
        |i|
        {
            i.2.to_rgb().0
        }
    ).collect();

    let mut rgb_reduced: Vec<[u16;3]> = Vec::with_capacity(rgb.len());
    let mut i: usize = 0;


    while i < rgb.len() - width as usize -1{
        let first = rgb[i];
        let second= rgb[i+1];
        let third = rgb[i+width as usize];
        let fourth = rgb[i+width as usize + 1];
        
        let down_sampled=
            [
        (first[0] as u16 + second[0] as u16 + third[0] as u16 + fourth[0] as u16) >> 2,
        (first[1] as u16 + second[1] as u16 + third[1] as u16 + fourth[1] as u16) >> 2,
        (first[2] as u16 + second[2] as u16 + third[2] as u16 + fourth[2] as u16) >> 2,
                ];

        rgb_reduced.push(down_sampled);

        if (i+2)%width as usize == 0{
            i= i+width as usize + 2;
        }else{
            i = i+2;
        }
        
    };


    let ascii: String = rgb_reduced
                                            .into_iter()
                                            .map(
                                                |i|
                                                if i[0] + i[1] + i[2] < 765 && i[0] + i[1] + i[2] > 588{
                                                    "▓".truecolor(i[0] as u8, i[1] as u8, i[2] as u8)
                                                }else if i[0] + i[1] + i[2] < 588 && i[0] + i[1] + i[2] > 460{
                                                    "▒".truecolor(i[0] as u8, i[1] as u8, i[2] as u8)
                                                }else{
                                                     "░".truecolor(i[0] as u8, i[1] as u8, i[2] as u8)
                                                }
                                                
                                            )
                                            .enumerate()
                                            .map(|(i, s)| 
                                            if i%(width as usize / 2) == 0{
                                                "\n".to_string()
                                            }else{
                                                s.to_string()
                                            }
                                            )
                                            .collect();


    println!("{}",ascii);
}
