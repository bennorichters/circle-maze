fn main() {
    if std::env::var("TARGET").unwrap().starts_with("wasm32") {
        println!("cargo:rustc-cfg=getrandom_backend=\"wasm_js\"");
    }
}
