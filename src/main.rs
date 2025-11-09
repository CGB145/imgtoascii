use image::{DynamicImage, GenericImageView, ImageBuffer, ImageEncoder, ImageReader, Pixel, Pixels, Rgb, RgbImage, Rgba, imageops::FilterType::Nearest};
use colored::{self, Colorize};
fn main() {
    let img_path: String =String::from("myimage.jpg");
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

    // this whole part is basically useless for this programm 
    let width: u32 = 100;
    let height: u32 = 100;
    let img_resized = img.resize_exact(width, height,Nearest);
    let rgb: Vec<_>  = img_resized.pixels().map(
        |i|{i.2.to_rgb().0}
    ).collect();

    let mut rgb_width_reduced: Vec<[u16;3]> = vec![];
    let mut prior_value: [u8;3] = [0,0,0];
    let mut j: usize = 0;

    for (i, val) in rgb.iter().enumerate(){
       // println!("i: {}, j: {}",i,j);
        if i == 0{
            prior_value = *val;
            j = i+1;
        }

        if i == j{
            rgb_width_reduced.push([ ((val[0] as u16 +prior_value[0] as u16)/2 ), ((val[1] as u16 +prior_value[1] as u16)/2), ((val[2] as u16+prior_value[2] as u16)/2) ])
        }

        if i > j{
             prior_value = *val;
            j = i+1;
        }
      //println!("i: {}, j: {}",i,j);
        //new_test.push( [val[0]+1, val[1]+1, val[2]+1]);
    }
    let mut i =0;
     let mut rgb_height_reduced: Vec<[u16;3]> = vec![];
    while i != rgb_width_reduced.len(){
        
         //println!("{:?}",i);
         //println!("{}",new_test.len());
         if i == rgb_width_reduced.len() - width as usize/2{
            break
         }
         //println!("first: {:?}, second:{:?}",new_test[i],new_test[i+width as usize/2]);
         //println!("{}", new_test[i][0]);

         rgb_height_reduced.push(
            [
                (( rgb_width_reduced[i][0] + rgb_width_reduced[i+width as usize/2][0])/2),
                 (( rgb_width_reduced[i][1] + rgb_width_reduced[i+width as usize/2][1])/2),
                 (( rgb_width_reduced[i][2] + rgb_width_reduced[i+width as usize/2][2])/2),
            ]
         );

        if i  == rgb_width_reduced.len() - width as usize / 2{
            break;
        }else if i % (width as usize /2) == width as usize/2 -1 {
            i = i +width as usize/2;
        }

        //println!("{}",i);
        
        i = i+1;
    }

    // useless part is over

    let ascii: String = rgb_height_reduced
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
    //println!("{}", width);



    //println!("{:?}",test);
    //println!("{:?}",new_test);
    //println!("{:?}", new_test_2);
}
