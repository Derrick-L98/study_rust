// use std::env;
// use std::path::PathBuf;

fn main() {
//     // Tell cargo to look for shared libraries in the specified directory
//     // 告诉cargo在指定目录中查找共享库
//     println!("cargo:rustc-link-search=/path/to/lib");

//     // Tell cargo to tell rustc to link the system bzip2
//     // shared library.
//     //告诉cargo告诉rustc链接系统bzip2共享库。
//     println!("cargo:rustc-link-lib=bz2");

//     // The bindgen::Builder is the main entry point
//     // to bindgen, and lets you build up options for
//     // the resulting bindings.
//     //bindgen:：Builder是bindgen的主要入口点，允许您为生成的绑定构建选项。
//     let bindings = bindgen::Builder::default()
//         // The input header we would like to generate
//         // bindings for.
//         //我们要为其生成绑定的输入标头。
//         .header("wrapper.h")
//         // Tell cargo to invalidate the built crate whenever any of the
//         // included header files changed.
//         //当任何包含的头文件发生变化时，告诉货物使构建的板条箱失效。
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
//         // Finish the builder and generate the bindings.
//         //完成构建器并生成绑定。
//         .generate()
//         // Unwrap the Result and panic on failure.
//         // 打开“结果”并对失败感到恐慌。
//         .expect("Unable to generate bindings");

//     // Write the bindings to the $OUT_DIR/bindings.rs file.
//     // 将绑定写入$OUT_DIR/bindings.rs文件。
//     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//     bindings
//         .write_to_file(out_path.join("bindings.rs"))
//         .expect("Couldn't write bindings!");
}