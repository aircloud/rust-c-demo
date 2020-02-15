error:
ld: symbol(s) not found for architecture x86_64

cargo rustc -- -C link-args="-Wl,-rpath,/Users/niexiaotao/work/rust-server/rust-ffi/hello-from-generated-code-3/lib"
$ RUSTFLAGS="-C link-args=-Wl,-rpath,/Users/niexiaotao/work/rust-server/rust-ffi/hello-from-generated-code-3/lib" cargo build

LD_LIBRARY_PATH=/Users/niexiaotao/work/rust-server/rust-ffi/hello-from-generated-code-3/lib cargo run