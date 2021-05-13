fn main() {
  let nginx_dir = "nginx-1.19.3";

  let bindings = bindgen::builder()
    .header("wrapper.h")
    .clang_args(vec![
      format!("-I{}/src/core", nginx_dir),
      format!("-I{}/src/event", nginx_dir),
      format!("-I{}/src/event/modules", nginx_dir),
      format!("-I{}/src/os/unix", nginx_dir),
      format!("-I{}/objs", nginx_dir),
      format!("-I{}/src/http", nginx_dir),
      format!("-I{}/src/http/v2", nginx_dir),
      format!("-I{}/src/http/modules", nginx_dir),
    ])
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file("nginx.rs")
    .expect("unable to write bindings");
}
