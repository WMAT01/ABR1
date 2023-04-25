//use wasm_bindgen::prelude::*;

// Your text diff implementation using the text_diff_lib
/*pub fn text_diff(/* Your input parameters */) {
    // Your implementation here
}*/
use difference::Changeset;

pub fn generate_text_diff_html(old: &str, new: &str) -> String {
    let changeset = Changeset::new(old, new, "\n");

    let mut html = String::new();
    html.push_str("<pre>");

    for diff in changeset.diffs {
        match diff {
            difference::Difference::Same(ref content) => {
                html.push_str(content);
            }
            difference::Difference::Add(ref content) => {
                html.push_str("<span class='added'>");
                html.push_str(content);
                html.push_str("</span>");
            }
            difference::Difference::Rem(ref content) => {
                html.push_str("<span class='removed'>");
                html.push_str(content);
                html.push_str("</span>");
            }
        }
    }

    html.push_str("</pre>");
    html
}
