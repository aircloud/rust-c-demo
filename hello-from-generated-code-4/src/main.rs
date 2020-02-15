extern crate libloading as lib;

fn call_dynamic() -> lib::Result<u32> {
    let lib = lib::Library::new("/Users/niexiaotao/work/rust-server/rust-ffi/hello-from-generated-code-4/lib/libadd.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn(a: i32, b: i32) -> i32> = lib.get(b"add")?;
        let c = func(3, 5);
        println!("c: {:?}", c);
        Ok(1)
    }
}

fn main() {
    let res = call_dynamic();
    println!("res: {:?}", res);
}
