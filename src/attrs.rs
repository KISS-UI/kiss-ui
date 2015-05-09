c_str_consts! {
    //Globals
    UTF8_MODE = "UTF8MODE",

    // Basic widget attributes
    TITLE = "TITLE",
    VALUE = "VALUE",
    ACTIVE = "ACTIVE",
    NAME = "NAME",

    // Rendering attributes
    RASTERSIZE = "RASTERSIZE",


    // Layout attributes
    ALIGNMENT_VERT = "ALIGNMENTLIN",
    ALIGNMENT_HORI = "ALIGNMENTCOL",
    ORIENTATION = "ORIENTATION",

    //Textbox attributes
    MULTILINE = "MULTILINE",
    VISIBLE_COLUMNS = "VISIBLECOLUMNS",
    VISIBLE_LINES = "VISIBLELINES",

    // Spacing between elements in a container
    GAP = "GAP",

    // Handles
    IMAGE = "IMAGE",

    //Callbacks
    ACTION = "ACTION",
    VALUE_CHANGED_CB = "VALUECHANGED_CB",
}

pub mod values {
    c_str_consts! {
        YES = "YES",
        NO = "NO",
    }

    pub fn bool_yes_no(_bool: bool) -> &'static str {
        match _bool {
            true => YES,
            false => NO,
        }
    }
}
