use presentation::App;

use dioxus_web::Config;

#[cfg(not(feature = "ssr"))]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch_cfg(App, Config::new().with_default_panic_hook(true));
}

#[cfg(feature = "ssr")]
fn main() {
    use dioxus::prelude::*;
    use wasm_bindgen::UnwrapThrowExt;

    wasm_logger::init(wasm_logger::Config::default());

    let mut dom = VirtualDom::new(App);
    let _ = dom.rebuild();

    let pre = dioxus_ssr::pre_render(&dom);

    // プリレンダリングされた内容をmainの内部htmlに挿入
    gloo_utils::document()
        .get_element_by_id("main")
        .unwrap_throw()
        .set_inner_html(&pre);

    // リハイドレーション
    // now rehydrate
    dioxus_web::launch_cfg(
        App,
        Config::new().with_default_panic_hook(true).hydrate(true),
    );
}
