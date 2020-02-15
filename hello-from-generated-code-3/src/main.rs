extern { fn add(a: i32, b: i32) -> i32; }

fn main() {
    let c = unsafe { let d = add(3, 5); d };
    println!("c: {:?}", c);
}