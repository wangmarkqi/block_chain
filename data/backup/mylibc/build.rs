extern crate bindgen;

use std::env;
use std::path::PathBuf;
//加载libload so不需要buildrs，直接使用cc编译c源码是下面的方式build_c，这个是使用bindgen根据wrapp h生成rust接口文件bindings rs，编译需要设定库路径，运行如果是动态链接
// export LD_LIBRARY_PATH=你的库的路径:$LD_LIBRARY_PATH
// echo $LD_LIBRARY_PATH

fn main() {
    bindgen_gen_binding_rs();
    build_c();
}

#[allow(dead_code)]
fn set_compile_link_lib() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib = format!("{}/src/lib/", root);
    println!("cargo:warning=MESSAGE");
    println!("cargo:rustc-link-search=all={}", lib);
    println!("cargo:rustc-link-lib=static=wq");
    // println!("cargo:rustc-link-lib=dylib=wq");
}

#[allow(dead_code)]
fn bindgen_gen_binding_rs() {
    let key = "OUT_DIR";
    std::env::set_var(key, "./src/mylibc");
    let bindings = bindgen::Builder::default()
        .header("./src/mylibc/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("**************outdir={:?}", out_path);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// #和compile with cc 配合使用，lib文件夹中
#[allow(dead_code)]
fn build_c() {
    cc::Build::new()
        .file("src/mylibc/rsa.c")
        .compile("librsa.a");
}