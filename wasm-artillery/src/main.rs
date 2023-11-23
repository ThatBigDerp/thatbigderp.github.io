use bevy_math::{uvec2, URect};
use pixel_map::{PixelMap, ILine};
use bitflags::bitflags;
use macroquad::prelude::*;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Pixel: u32 {
        const ALIVE = 0b0000_0001;
        const DEAD = 0b0000_0000;
        const WAS_ALIVE = 0b0000_0100;

        const TERRAIN = Self::ALIVE.bits() | Self::WAS_ALIVE.bits();
        const AIR = Self::DEAD.bits();
    }
}

#[macroquad::main("wasm-artillery")]
async fn main() {

    // MOCK RANDOM TERRAIN GENERATION
    let mut noise: Vec<u32> = vec![];
    let max: u32 = 350;
    let min: u32 = 250;
    for i in 0..=100 {
        noise.push((i as u32)+250);
    }
    for i in (0..100).rev() {
        noise.push((i as u32)+250)
    }

    // TERRAIN PIXEL MAP
    let mut pixel_map = PixelMap::<Pixel>::new(
        &uvec2(500, 500), // size of the pixel map
        Pixel::AIR,     // initial value
        1,                  // pixel size
    );
// FROM 1D NOISE TO 2D TERRAIN
pixel_map.draw_rect(&URect::from_corners((0, 0).into(), (500, min).into()), Pixel::TERRAIN);
for i in noise {
   let mut j = 0;
    pixel_map.draw_rect(&URect::from_corners((j, i).into(), (j, min).into()), Pixel::TERRAIN);
    j+=1;




    let mut image = Image::gen_image_color(500 as u16, 500 as u16, SKYBLUE);
    let texture = Texture2D::from_image(&image);




    loop {
        clear_background(WHITE);

        for i in 0..=500 {
            for j in 0..=500 {
                    match pixel_map.get_pixel(uvec2(i, j)) {
                    Some(&Pixel::TERRAIN) => image.set_pixel(i, j, BROWN),
                    Some(&Pixel::AIR) =>  image.set_pixel(i, j, SKYBLUE),
                    None => image.set_pixel(i, j, SKYBLUE),
                    _ => image.set_pixel(i, j, PINK)
                }
            }
        }



        texture.update(&image);

        draw_texture(&texture, 0., 0., WHITE);

        next_frame().await
    }
}


}