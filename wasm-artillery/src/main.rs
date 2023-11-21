use simdnoise::*;
use bevy_math::{uvec2, URect};
use pixel_map::PixelMap;
use bitflags::bitflags;

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

fn main() {

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
    print!("{:?}", noise);
// TERRAIN PIXEL MAP
    let mut pixel_map = PixelMap::<Pixel>::new(
        &uvec2(500, 500), // size of the pixel map
        Pixel::AIR,     // initial value
        1,                  // pixel size
    );
// FROM 1D NOISE TO 2D TERRAIN
pixel_map.draw_rect(&URect::from_corners((0, 0).into(), (500, min).into()), Pixel::TERRAIN);
for i in 0..=500 {
    
}
}