use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};


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
let w = 128;
let h = 64;
let M = 4294967296;
let A = 1664525;
let C = 1;
let Z = 
    let mut terrain: Quadtree<u64, Pixel> = Quadtree::<u64, Pixel>::new(4);
    
}



// w = window.innerWidth,
// 		h = window.innerHeight;
// canvas.width = w;
// canvas.height = h;

// var M = 4294967296,
//     // a - 1 should be divisible by m's prime factors
//     A = 1664525,
//     // c and m should be co-prime
//     C = 1;
// var Z = Math.floor(Math.random() * M);
// function rand(){
//   Z = (A * Z + C) % M;
//   return Z / M - 0.5;
// };

// function interpolate(pa, pb, px){
// 	var ft = px * Math.PI,
// 		f = (1 - Math.cos(ft)) * 0.5;
// 	return pa * (1 - f) + pb * f;
// }

// var x = 0,
// 	y = h / 2,
// 	amp = 100, //amplitude
// 	wl = 100, //wavelength
// 	fq = 1 / wl, //frequency
// 	a = rand(),
// 	b = rand();

// while(x < w){
// 	if(x % wl === 0){
// 		a = b;
// 		b = rand();
// 		y = h / 2 + a * amp;
// 	}else{
// 		y = h / 2 + interpolate(a, b, (x % wl) / wl) * amp;
// 	}
// 	ctx.fillRect(x, y, 1, 1);
// 	x += 1;
// }