use presentation::App;

use dioxus::prelude::*;
use std::io::Write;

fn main() {
    let mut vdom = VirtualDom::new(App);
    let _ = vdom.rebuild();

    let text = dioxus_ssr::render(&vdom);
    let mut file = std::fs::File::create("rendered.html").unwrap();
    file.write_all(text.as_bytes()).unwrap();
}
