extern crate pkg_config;

use std::collections::HashSet;
use std::path::PathBuf;
use cc;
use glob::glob;
use std::path::Path;


fn main() {

    add_lib("zxing","0.29");
    println!("cargo:include=/usr/include/stb/");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=dylib=stb");

    //add_lib("stb","");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    let c_api_src_dir = Path::new("c_src");
    let c_api_sources: Vec<_> = glob(c_api_src_dir.join("*.cpp").to_str().unwrap())
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .flag("-v")
        .flag("-g")
        .include(c_api_src_dir)
        .files(c_api_sources)
        .compile("zxing_c_api");
}




fn add_lib(libname:&'static str,ver:&'static str) {
    let pkg=run_pkg_config(libname,ver);
    let lib_dirs = pkg.link_paths;
    for d in &lib_dirs {
        if !d.exists() {
            panic!("{} library directory does not exist: {}",libname, d.to_string_lossy());
        }
        println!("cargo:rustc-link-search=native={}", d.to_string_lossy());
    }
    let include_dirs = pkg.include_paths;
    for d in &include_dirs {
        if !d.exists() {
            panic!("{} include directory does not exist: {}",libname, d.to_string_lossy());
        }
        println!("cargo:include={}", d.to_string_lossy());
    }
    let libs = pkg.libs;
    let kind = determine_mode(libname,&lib_dirs, libs.as_slice());
    for lib in libs.into_iter() {
        println!("cargo:rustc-link-lib={}={}", kind, lib);
    }
}


fn determine_mode<T: AsRef<str>>(libname:&'static str,libdirs: &[PathBuf], libs: &[T]) -> &'static str {
    let files = libdirs
        .iter()
        .flat_map(|d| d.read_dir().unwrap())
        .map(|e| e.unwrap())
        .map(|e| e.file_name())
        .filter_map(|e| e.into_string().ok())
        .collect::<HashSet<_>>();
    let can_static = libs.iter().all(|l| {
        files.contains(&format!("lib{}.a", l.as_ref()))
            || files.contains(&format!("{}.lib", l.as_ref()))
    });
    let can_dylib = libs.iter().all(|l| {
        files.contains(&format!("lib{}.so", l.as_ref()))
            || files.contains(&format!("{}.dll", l.as_ref()))
            || files.contains(&format!("lib{}.dylib", l.as_ref()))
    });

    match (can_static, can_dylib) {
        (true, false) => return "static",
        (false, true) => return "dylib",
        (false, false) => {
            panic!(
                "{} libdirs at `{:?}` do not contain the required files \
                 to either statically or dynamically link ZXing",
                libname,
                libdirs
            );
        }
        (true, true) => {}
    }
    "dylib"
}

fn run_pkg_config(lib:&'static str,ver:&'static str) -> pkg_config::Library {
    pkg_config::Config::new()
        .cargo_metadata(false)
        .atleast_version(ver)
        .probe(lib)
        .unwrap();
    pkg_config::Config::new().cargo_metadata(false).probe(lib).unwrap()
}