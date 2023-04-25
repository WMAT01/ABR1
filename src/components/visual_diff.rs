/**
visual_diff.rs:
For visual diffs, you'll need to use a third-party library, such as resemblejs, which is a JavaScript library. You can use the wasm_bindgen and js_sys crates to run JavaScript code in your Rust application. First, include resemblejs in your project by adding it to your index.html:
html
Copy code
<script src="https://cdnjs.cloudflare.com/ajax/libs/resemblejs/3.2.0/resemble.js"></script>
**/
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
    export function generate_visual_diff(old_image, new_image, callback) {
        const options = {
            scaleToSameSize: true,
            ignore: 'less'
        };

        resemble(old_image).compareTo(new_image).withOptions(options).onComplete(callback);
    }
"#)]
extern "C" {
    pub fn generate_visual_diff(old_image: &str, new_image: &str, callback: &Closure<dyn FnMut(JsValue)>);
}

