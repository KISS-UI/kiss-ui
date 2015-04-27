macro_rules! def_attr (
    ($name:ident = $val:expr) => (
        pub const $name: &'static str = concat!($val, "\0");
    )
);

macro_rules! def_attrs {
    ($($name:ident = $val:expr),*) => (
        $(def_attr!($name = $val);)*
    )
}

def_attrs! {
    // Basic widget attributes
    TITLE = "TITLE",

    // Rendering attributes
    RASTERSIZE = "RASTERSIZE",


    // Layout attributes
    ALIGNMENT_VERT = "ALIGNMENTLIN",
    ALIGNMENT_HORI = "ALIGNMENTCOL",
    ORIENTATION = "ORIENTATION",

    IMAGE = "IMAGE"
}
