use presentation::{App, AppProps};

use dioxus_web::Config;
use toml::Table;

fn get_admin_password(toml_str: &str) -> String {
    let table = toml_str.parse::<Table>().expect("Cannot Read Secrets.toml");
    table
        .get("admin_password")
        .expect("Cannot Get admin_password from Secrets.toml")
        .as_str()
        .expect("admin_password is invalid type.")
        .to_string()
}

#[cfg(not(feature = "ssr"))]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch_with_props(
        App,
        AppProps {
            admin_password: admin_password,
        },
        Config::new().with_default_panic_hook(true),
    );
}

#[cfg(feature = "ssr")]
fn main() {
    // use dioxus::prelude::*;
    use wasm_bindgen::UnwrapThrowExt;

    wasm_logger::init(wasm_logger::Config::default());

    log::info!("リハイドレーションを開始");

    // admin_passwordの取得
    let admin_password = get_admin_password(include_str!("../../Secrets.toml"));

    // let mut dom = VirtualDom::new(App);
    // let _ = dom.rebuild();

    // let pre = dioxus_ssr::pre_render(&dom);

    // // プリレンダリングされた内容をmainの内部htmlに挿入
    // gloo_utils::document()
    //     .get_element_by_id("main")
    //     .unwrap_throw()
    //     .set_inner_html(&pre);

    // // リハイドレーション
    // dioxus_web::launch_cfg(
    //     App,
    //     Config::new().with_default_panic_hook(true).hydrate(true),
    // );

    // プリレンダリングされた内容をmainの内部htmlに挿入
    gloo_utils::document()
        .get_element_by_id("main")
        .unwrap_throw()
        .set_inner_html("");

    dioxus_web::launch_with_props(
        App,
        AppProps {
            admin_password: admin_password,
        },
        Config::new().with_default_panic_hook(true),
    );
}
