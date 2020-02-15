extern { fn add(a: i32, b: i32) -> i32; }

fn main() {
    let c = unsafe { add(3, 5); };
    println!("c: {:?}", c);
}