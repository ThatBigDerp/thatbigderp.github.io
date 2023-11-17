
mod terrain_gen;
pub use terrain_gen::Terrain;
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

    let terrain: Terrain::new(500,500);
    
}


