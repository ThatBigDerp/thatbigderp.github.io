use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};


mod noise;
pub use noise::PerlinNoise;

pub struct Terrain {
    quadtree: Quadtree,
    width: u32,
    height: u32,

}
impl Terrain {
    pub fn new(width: u32, height: u32) -> Terrain {
        let mut quadtree: Quadtree<u64, Pixel> = Quadtree::<u64, Pixel>::new(4);
        let perlin = PerlinNoise::new();
        let mut TerrainHeight: [f64; width];

        for i in 0..width/2 {
            let value = perlin.get((i as f64) / 10.0);
            TerrainHeight[i] = value;
            print!("{}: {:.1}", i, value);
        }
        for i in (0..width/2).rev() {
            let value = perlin.get((i as f64) / 10.0);
            TerrainHeight[width/2+i] = value;
            print!("{}: {:.1}", i, value);
        }
        
        Terrain {
            quadtree: quadtree,
            width: width,
            height: height
        }
    }
}