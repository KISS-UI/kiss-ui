c_str_consts! {
    //Globals
    UTF8_MODE = "UTF8MODE",

    // Basic widget attributes
    TITLE = "TITLE",
    VALUE = "VALUE",
    ACTIVE = "ACTIVE",
    NAME = "NAME",
    VISIBLE = "VISIBLE",

    // Rendering attributes
    RASTERSIZE = "RASTERSIZE",
    POSITION = "POSITION",

    // Layout attributes
    ALIGNMENT_VERT = "ALIGNMENTLIN",
    ALIGNMENT_HORI = "ALIGNMENTCOL",
    ORIENTATION = "ORIENTATION",
    NUMDIV = "numdiv",

    //Textbox attributes
    MULTILINE = "MULTILINE",
    VISIBLE_COLUMNS = "VISIBLECOLUMNS",
    VISIBLE_LINES = "VISIBLELINES",

    // Progressbar attributes
    DASHED = "DASHED",
    MARQUEE = "MARQUEE",
    MIN = "MIN",
    MAX = "MAX",

    //Timer attribute
    TIME = "TIME",
    RUN = "RUN",

    // Spacing between elements in a container
    GAP = "GAP",

    // Handles
    IMAGE = "IMAGE",

    //Callbacks
    ACTION = "ACTION",
    ACTION_CB = "ACTION_CB",
    VALUE_CHANGED_CB = "VALUECHANGED_CB",
    MAP_CB = "MAP_CB",
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
