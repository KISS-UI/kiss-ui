fn main() {
    add_link("iup");

    if cfg!(not(windows)) {
        add_link("gtk-3"); 
    }
}

fn add_link(link: &str) {
    println!("cargo:rustc-link-lib={}", link);
}
