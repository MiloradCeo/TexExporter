use macroquad::prelude::*;
use std::fs::File;
use std::io::{Read};
use walkdir::WalkDir;


#[macroquad::main("Ratbag .tex exporter")]
async fn main() {
    let mut files = 0;
    let mut name = "".to_string();
    for file in WalkDir::new("./data").into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            name = file.path().display().to_string();
            if &name[name.len()-3..name.len()] == "tex"{
                println!("{}", name);










                let mut loadpoint:i32;
                let mut out: u16;
                let mut bit = 0;
                let mut bitw = 0;
                let mut r:u8;
                let mut g:u8;
                let mut b:u8;
                let mut a:u8;
                let mut x = 0;
                let mut y = 0;
                let mut i = 0;

                out = 0;
                loadpoint = 32;
                let mut path = String::from(name);
                let mut file = File::open(&path);
                let mut buffer = Vec::new();
                file.expect("REASON").read_to_end(&mut buffer);
                println!("File read successfully, bytes: {}", buffer.len());

                let mut colour = Color::new(1.00, 0.43, 0.76, 1.00);
                //get width
                let w:u16 = ((((buffer[21] as u16) << 8) | buffer[20] as u16) as usize).try_into().unwrap(); // Source - https://stackoverflow.com/a/50244328, Posted by Shepmaster, modified by ME. See post 'Timeline' for change history, Retrieved 2026-03-03, License - CC BY-SA 4.0
                //get width
                let h:u16 = ((((buffer[25] as u16) << 8) | buffer[24] as u16) as usize).try_into().unwrap(); // Source - https://stackoverflow.com/a/50244328, Posted by Shepmaster, modified by ME. See post 'Timeline' for change history, Retrieved 2026-03-03, License - CC BY-SA 4.0

                let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
                let texture = Texture2D::from_image(&image);
                let mut oy = h as u32;

                println!("numba{}", buffer[28]);
                if buffer[28] == 32 {
                    //32 bit loader
                    //rrrrrrrrggggggggbbbbbbbbaaaaaaaa
                    while y <= (h-1) as u32{


                        b = buffer[loadpoint as usize];
                        loadpoint +=1;
                        g = buffer[loadpoint as usize];
                        loadpoint +=1;
                        r = buffer[loadpoint as usize];
                        loadpoint +=1;
                        a = buffer[loadpoint as usize];
                        loadpoint +=1;
                        colour = Color::new(r as f32 / 255., g as f32 / 255., b as f32 / 255., a as f32 /  255.);

                        image.set_pixel(x, oy -1, colour);

                        x+=1;
                        if x == w as u32{
                            x = 0;
                            y +=1;
                            oy -=1;
                        }




               }

                    println!("hel");
                    println!("w{}", w);
                    println!("h{}", h);
                    path.remove(path.len() - 1).to_string();
                    path.remove(path.len() - 1).to_string();
                    path.remove(path.len() - 1).to_string();
                    path += "png";
                    println!("{}", path);
                    &image.export_png(&path);
                    clear_background(BLACK);
                    texture.update(&image);
                    files +=1;
                    draw_text("Powerslide & Ratbag games .TEX file exporter made by Milorad", 16., 32., 16.0, WHITE);
                    draw_text(&path as &str, 16., 64., 16.0, WHITE);
                    draw_text("files read:", 650., 64., 16.0, WHITE);
                    draw_text(&files.to_string(), 740., 64., 16.0, WHITE);
                    draw_texture_ex(&texture, 400. - 512 as f32/2., 332. - 512 as f32/2., WHITE, DrawTextureParams {
                        dest_size:Some(vec2(512., 512.)),
                                    flip_y:true,
                                    ..Default::default()
                    });
                    next_frame().await; //RENDER IT!
                }else{
                    //16bit loader
                    //rrrrrggggggbbbbb
                    //data extractor
                    while y <= (h-1) as u32{
                        //group two 8-byte chunks into a 16-byte word
                        out = ((buffer[(loadpoint + 1) as usize] as u16) << 8) | buffer[(loadpoint+0) as usize] as u16; // Source - https://stackoverflow.com/a/50244328, Posted by Shepmaster, modified by ME. See post 'Timeline' for change history, Retrieved 2026-03-03, License - CC BY-SA 4.0
                        b = ((out & 0b11111) << 3) as u8;
                        g = ((out & 0b11111100000) >> 3) as u8;
                        r = ((out & 0b1111100000000000) >> 8) as u8;



                        colour = Color::new(r as f32 / 255., g as f32 / 255., b as f32 / 255., 1.00);

                        image.set_pixel(x, oy -1, colour);


                        loadpoint +=2;
                        x+=1;
                        if x == w as u32{
                            x = 0;
                            y +=1;
                            oy -=1;
                        }

                    }










                    println!("hel");

                    println!("w{}", w);
                    println!("h{}", h);
                    path.remove(path.len() - 1).to_string();
                    path.remove(path.len() - 1).to_string();
                    path.remove(path.len() - 1).to_string();
                    path += "png";
                    println!("{}", path);
                    &image.export_png(&path);
                    clear_background(BLACK);
                    texture.update(&image);
                    files+=1;
                    draw_text("Powerslide & Ratbag games .TEX file exporter made by Milorad", 16., 32., 16.0, WHITE);
                    draw_text(&path as &str, 16., 64., 16.0, WHITE);
                    draw_text("files read:", 650., 64., 16.0, WHITE);
                    draw_text(&files.to_string(), 740., 64., 16.0, WHITE);
                    draw_texture_ex(&texture, 400. - 512 as f32/2., 332. - 512 as f32/2., WHITE, DrawTextureParams {
                        dest_size:Some(vec2(512., 512.)),
                                    flip_y:true,
                                    ..Default::default()
                    });
                    next_frame().await; //RENDER IT!
                }
            }








            }
        }


        loop{
            clear_background(BLACK);
            draw_text("Powerslide & Ratbag games .TEX file exporter made by Milorad", 16., 32., 16.0, WHITE);
            draw_text("Thank you for using this software and enjoy your assets! you may now close the program", 16., 64., 16.0, WHITE);
            draw_text("files read:", 650., 64., 16.0, WHITE);
            draw_text(&files.to_string(), 740., 64., 16.0, WHITE);

            next_frame().await; //RENDER IT!
        }

    }
