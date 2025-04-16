pub fn add_label(s_path: &String, label_pos: i8, is_hidden: bool, is_file: bool, is_symlink: bool) -> String {
    let label = generate_label(is_hidden, is_file, is_symlink);
    if label_pos == 1 {
        return format!("{} {}", s_path, label);
    }
    return format!("{} {}", label, s_path);
}

pub fn generate_label(is_hidden: bool, is_file: bool, is_symlink: bool) -> String {
    let mut ret = String::from("FRR");
    if !is_file {
        ret.replace_range( 0..2, "D_");
    }
    if is_symlink {
        ret.replace_range( 1..2, "S");
    }
    if is_hidden {
        ret.replace_range( 2..3, "H");
    }
    return ret;
}