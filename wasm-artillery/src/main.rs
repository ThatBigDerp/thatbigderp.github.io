use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};



#[derive(PartialEq, Debug)]
struct Pixel {
    state: u8,
}


impl Pixel {
     fn new(alive: bool) -> Pixel {
        Pixel {
           state: 0b_0000_0000 | alive as u8
        }
    }
}


fn main() {
    let mut terrain: Quadtree<u64, Pixel> = Quadtree::<u64, Pixel>::new(4);
    
    let region_a = AreaBuilder::default()
    .anchor(Point {x: 0, y: 0})
    .dimensions((2, 1))
    .build().unwrap();
    terrain.insert(region_a, Pixel { state: (0b_0000_0001) });


        // Query over a region of size 2x2, anchored at (1, 0).
    let region_b = AreaBuilder::default()
    .anchor(Point {x: 1, y: 0})
    .dimensions((2, 2))
    .build().unwrap();
    let mut query = terrain.query(region_b);
    let mut thepixel: Pixel = Pixel::new(true);

    // The query region (region_b) intersects the region "foo" is associated with (region_a), so the query iterator returns "foo" by reference.
    assert_eq!(query.next().unwrap().value_ref(), &thepixel);
}
