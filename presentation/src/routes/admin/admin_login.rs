use crate::AdminPassword;

use dioxus::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Props)]
pub struct AdminLoginProps<'a> {
    on_cancel: EventHandler<'a>,
}

pub fn AdminLogin<'a>(cx: Scope<'a, AdminLoginProps<'a>>) -> Element {
    let value_state = cx.use_hook(|| Rc::new(Cell::new("".to_string())));
    let correct_value = use_shared_state::<AdminPassword>(cx)
        .expect("Cannot get ShareState: AdminPassword")
        .read()
        .0
        .clone();

    let submit_password = {
        to_owned![value_state];
        move |_| {
            let value = value_state.take();
            if value == correct_value {
                cx.props.on_cancel.call(());
            }
        }
    };

    cx.render(rsx!{
        div {id: "admin-login-container",
            div {id: "admin-login-ui-container",
                div {id: "admin-login-caption", "管理者ページへログイン"}
                div {id: "admin-login-input-container",
                    div {"パスワード"}
                    input {id: "admin-login-password", oninput: move |e|{value_state.set(e.value.clone())}}
                }
                div {id: "admin-login-bottom",
                    button {onclick: submit_password, "ログイン"}
                }
            }
        }

    })
}
