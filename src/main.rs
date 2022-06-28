#[derive(Debug)]

struct Cuboid {
    length: u32,
    breadth: u32,
    height: u32,
}

fn main() {
    let cuboid1 = Cuboid {
        length: 40,
        breadth: 30,
        height: 50,
    };

    println!(" Cuboid dimensions are {:?}", cuboid1);
}