use std::env;

fn main() {
    match env::var("$RUST_PI_COMPILATION") {
        Ok(val) => println!("cargo:rustc-cfg={:?}", val),
        Err(e) => {
            println!("cargo:warning=WARNING: RASPBERRY PI NOT DETECTED. SKIPPING RPI ONLY MODULES. {:?}",
                     e)
        } //Pi not detected
    }
}
