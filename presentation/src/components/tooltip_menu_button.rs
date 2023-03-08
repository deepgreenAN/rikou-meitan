use dioxus::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

// -------------------------------------------------------------------------------------------------
// TooltipMenuItem

#[derive(Props)]
pub struct TooltipMenuItem<'a> {
    children: Element<'a>,
}

pub fn TooltipMenuItem<'a>(cx: Scope<'a, TooltipMenuItem<'a>>) -> Element {
    cx.render(rsx! {
        div {class: "tooltip-menu-item",
            &cx.props.children
        }
    })
}

// -------------------------------------------------------------------------------------------------
// TooltipPos

#[derive(Clone, Copy, strum_macros::Display)]
enum TooltipPos {
    /// 通常通り右下に表示
    #[strum(serialize = "bottom-right")]
    BottomRight,
    /// 右端にある場合は左下に表示
    #[strum(serialize = "bottom-left")]
    BottomLeft,
    /// 下辺にある場合は右上に表示
    #[strum(serialize = "top-right")]
    TopRight,
    /// 右下にある場合は．左上に表示
    #[strum(serialize = "top-left")]
    TopLeft,
}

// -------------------------------------------------------------------------------------------------
// TooltipMenuButton

#[derive(Props)]
pub struct TooltipMenuButtonProps<'a> {
    children: Element<'a>,
}

pub fn TooltipMenuButton<'a>(cx: Scope<'a, TooltipMenuButtonProps<'a>>) -> Element {
    let is_menu_active = use_state(cx, || false);
    let tooltip_menu_pos = cx.use_hook(|| Rc::new(Cell::new(TooltipPos::BottomRight)));

    let dot_menu_svg_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/release/dot-menu.svg"
    ));

    let menu_width = 200; //px
    let menu_height = 120; //px

    let menu_button_on_click = {
        to_owned![tooltip_menu_pos];
        move |e: MouseEvent| {
            if !is_menu_active.get() {
                let document_element = gloo_utils::document_element();
                let (window_width, window_height) = (
                    document_element.client_width(),
                    document_element.client_height(),
                );
                // log::info!("window_width: {window_width}, window_height: {window_height}");

                let (x, y) = (
                    e.client_coordinates().x as i32,
                    e.client_coordinates().y as i32,
                );
                // log::info!("x: {x}, y: {y}");

                let pos = match (x + menu_width, y + menu_height) {
                    (right_x, bottom_y) if right_x < window_width && bottom_y < window_height => {
                        TooltipPos::BottomRight
                    } // 通常
                    (right_x, bottom_y) if right_x >= window_width && bottom_y < window_height => {
                        TooltipPos::BottomLeft
                    } // 右端
                    (right_x, bottom_y) if right_x < window_width && bottom_y >= window_height => {
                        TooltipPos::TopRight
                    } // 下辺
                    (right_x, bottom_y) if right_x >= window_width && bottom_y >= window_height => {
                        TooltipPos::TopLeft
                    } // 右下
                    (_, _) => unreachable!(),
                };

                tooltip_menu_pos.set(pos);
            }
            is_menu_active.modify(|flag| !flag);
        }
    };

    cx.render(rsx! {
        div { class: "tooltip-menu-container",
            div {
                class: "dot-menu-button",
                dangerous_inner_html: "{dot_menu_svg_str}",
                onclick: menu_button_on_click,
            }
            if *is_menu_active.get() {
                rsx!{
                    div { class: format_args!("tooltip-menu-items-container {}", tooltip_menu_pos.get()),
                        onclick: move |_| {is_menu_active.set(false)}, 
                        &cx.props.children
                    }
                    div { class: format_args!("tooltip-menu-cover {}", tooltip_menu_pos.get()),
                        onmouseout: move |_| {is_menu_active.set(false)}
                    }
                    div { class: "fixed-overlay", onclick: move |_|{is_menu_active.set(false);}}
                }
            }
        }
    })
}
