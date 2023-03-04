mod admin_login;

use crate::utils::use_overlay;
use admin_login::AdminLogin;

use dioxus::prelude::*;
use dioxus_router::Link;

pub fn AdminPage(cx: Scope) -> Element {
    let is_admin_login_open = use_state(cx, || true);
    let overlay_state = use_overlay(cx, 2);

    use_effect(cx, (), {
        to_owned![overlay_state];
        |_| async move {
            overlay_state.activate().expect("Cannot overlay activate");
        }
    });

    // 管理者ログインモーダルを閉じた処理
    let close_admin_login = move |_| {
        is_admin_login_open.set(false);
        overlay_state.deactivate();
    };

    cx.render(rsx! {
        is_admin_login_open.get().then(||{
            rsx!{
                AdminLogin{on_cancel: close_admin_login}
            }
        })

        div {id: "admin-menu-container",
            div {id: "admin-menu-caption", "メニュー"}
            Link{ to: "/admin/episodes", "エピソード"}
            Link{ to: "/admin/clips", "クリップ"}
        }
    })
}
