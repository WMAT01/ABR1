use htmldiff::diff;

pub fn generate_web_diff_html(old: &str, new: &str) -> String {
    diff(old, new)
}
