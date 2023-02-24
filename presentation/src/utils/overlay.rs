use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Event, HtmlElement};

/// オーバーレイを扱う状態
#[derive(Clone, Debug)]
pub struct OverlayElement {
    element: Rc<RefCell<Option<Element>>>,
    z_index: i32,
    event_listeners: Rc<RefCell<Vec<EventListener>>>,
}

impl OverlayElement {
    pub fn new(z_index: i32) -> Self {
        Self {
            element: Rc::new(RefCell::new(None)),
            z_index,
            event_listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// 状態のオーバーレイエレメントを取得
    pub fn element(&self) -> Option<Element> {
        // elementをクローンする．
        self.element.replace_with(|element| element.clone())
    }
    /// オーバーレイをアクティブ化する
    pub fn activate(&self) -> Result<(), JsValue> {
        let has_element = { self.element.borrow().is_some() }; // RefCellの再帰を防ぐ

        if !has_element {
            let element = gloo_utils::document().create_element("div")?;
            element.set_class_name("overlay");
            let element_style = element.clone().unchecked_into::<HtmlElement>().style();
            element_style.set_property("z-index", &format!("{}", self.z_index))?; // z-indexを設定
            let parent = gloo_utils::document()
                .query_selector("#background-container")?
                .ok_or(JsValue::from("Cannot get #background-container"))?;

            parent.append_child(element.as_ref())?;
            {
                let _ = self.element.borrow_mut().insert(element);
            }
        }

        Ok(())
    }
    /// オーバーレイにイベントリスナーを追加．アクティブ化してから利用する必要がある．
    pub fn add_event_listener<S: Into<Cow<'static, str>>, F: FnMut(&Event) + 'static>(
        &self,
        event_type: S,
        callback: F,
    ) -> Result<(), JsValue> {
        if let Some(element) = self.element.borrow().as_ref() {
            let event_listener = EventListener::new(element, event_type, callback);
            {
                self.event_listeners.borrow_mut().push(event_listener);
            }

            Ok(())
        } else {
            Err(JsValue::from("Overlay not activate error."))
        }
    }
    /// オーバーレイを非アクティブ化する
    pub fn deactivate(&self) {
        let element = self.element.borrow_mut().take();
        if let Some(element) = element {
            element.remove();
        }

        self.event_listeners.borrow_mut().clear();
    }
}

pub fn use_overlay<T>(cx: Scope<'_, T>, z_index: i32) -> &'_ OverlayElement {
    cx.use_hook(|| OverlayElement::new(z_index))
}
