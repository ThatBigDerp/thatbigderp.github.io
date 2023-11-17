use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};
extern crate rand;

mod noise;
pub use noise::PerlinNoise;

#[derive(PartialEq, Debug)]
struct Pixel {
    state: u8,
}


impl Pixel {
     fn new(alive: bool) -> Pixel {
        let mut state: u8 = 0b_0000_0000 | alive as u8;
        state = state | ((alive as u8) << 1);
        Pixel {
           state: state,
        }
    }
}


fn main() {
    let perlin = PerlinNoise::new();
    for i in 0..100 {
        let value = perlin.get((i as f64) / 10.0);
        println!("{}: {:.5}", i, value);
    }
    let mut terrain: Quadtree<u64, Pixel> = Quadtree::<u64, Pixel>::new(4);
    
}


