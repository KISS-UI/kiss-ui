fn main() {
    add_link("iup");
}

fn add_link(link: &str) {
    println!("cargo:rustc-link-lib={}", link);
}
