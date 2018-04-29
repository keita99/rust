fn hello_world() {
    println!("Hello, world!");
}

fn main() {
	for x in 1..10 {
		println!( x );
	}
}

fn prime_numberI( z: i32) {
	for x in 0..z {
		for y in 0..z {
			if z >= x * y { break; }
		}
	}    
}
