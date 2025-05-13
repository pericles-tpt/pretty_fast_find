const LABEL_DEFAULT: &str = "FRR";
pub const LABEL_LENGTH: usize = LABEL_DEFAULT.len();

pub fn add_label(s_path: &String, label_pos: i8, is_hidden: bool, is_file: bool, is_symlink: bool) -> String {
    let label = generate_label(is_hidden, is_file, is_symlink);
    if label_pos == 1 {
        return format!("{} {}", s_path, label);
    }
    return format!("{} {}", label, s_path);
}

pub fn generate_label(is_hidden: bool, is_file: bool, is_symlink: bool) -> String {
    let mut ret = String::from(LABEL_DEFAULT);
    if !is_file {
        ret.replace_range( 0..1, "D");
    }
    if is_symlink {
        ret.replace_range( 1..2, "S");
    }
    if is_hidden {
        ret.replace_range( 2..3, "H");
    }
    return ret;
}

pub fn remove_label_from_string(s: String, label_pos: i8) -> String {
    if label_pos == 0 || s.len() < (LABEL_LENGTH + 1) {
        return s;
    } else if label_pos > 0 {
        return s[..s.len() - (LABEL_LENGTH + 1)].to_string();
    }
    return s[(LABEL_LENGTH + 1)..].to_string();
}