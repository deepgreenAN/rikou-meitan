use presentation::{App, AppProps};

use dioxus_web::Config;
use toml::Table;
use wasm_bindgen::UnwrapThrowExt;

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
    console_log::init_with_level(log::Level::Info).unwrap_throw();
    dioxus_web::launch_with_props(
        App,
        AppProps {
            admin_password: get_admin_password(presentation::include_str_from_root!(
                "../Secrets.toml"
            )),
        },
        Config::new().with_default_panic_hook(true),
    );
}

#[cfg(feature = "ssr")]
fn main() {
    use dioxus::prelude::*;

    console_log::init_with_level(log::Level::Info).unwrap_throw();

    log::info!("リハイドレーションを開始");

    // admin_passwordの取得
    let admin_password =
        get_admin_password(presentation::include_str_from_root!("../Secrets.toml"));
    let app_props = AppProps {
        admin_password: admin_password,
    };

    let mut dom = VirtualDom::new_with_props(App, app_props.clone());
    let _ = dom.rebuild();

    let pre = dioxus_ssr::pre_render(&dom);

    // プリレンダリングされた内容をmainの内部htmlに挿入
    gloo_utils::document()
        .get_element_by_id("main")
        .unwrap_throw()
        .set_inner_html(&pre);

    // リハイドレーション
    dioxus_web::launch_with_props(
        App,
        app_props,
        Config::new().with_default_panic_hook(true).hydrate(true),
    );
}
