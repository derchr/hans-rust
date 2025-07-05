// use flate2::read::GzDecoder;
use std::{env, fs::File, path::PathBuf};
// use tar::Archive;

fn main() {
    let hall_dir = PathBuf::from("hall");
    println!("cargo:rustc-link-search=native={}", hall_dir.display());
    println!("cargo:rustc-link-lib=static=Hall");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header("hall/include/Hall/Audio.hpp")
        .header("hall/include/Hall/System.hpp")
        .header("hall/include/Hall/Video.hpp")
        .use_core()
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings");

    // let mut hall_resp =
    //     reqwest::blocking::get("https://github.com/derchr/Hall/archive/refs/tags/v2.1.1.tar.gz")
    //         .unwrap();
    // let hall_path = out_dir.join("v2.1.1.tar.gz");
    // let mut hall_tar_gz = File::create(&hall_path).unwrap();
    // std::io::copy(&mut hall_resp, &mut hall_tar_gz).unwrap();

    // let hall_tar_gz = File::open(&hall_path).unwrap();
    // let hall_tar = GzDecoder::new(hall_tar_gz);
    // let mut hall_archive = Archive::new(hall_tar);
    // hall_archive.unpack(&out_dir).unwrap();

    // let hall_cmake_dir = out_dir.join("Hall-2.1.1");
    // let dst = cmake::build(hall_cmake_dir);
    // println!("cargo:warning={hall_path:?}");

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-arg=-Tmemory.x");
}
