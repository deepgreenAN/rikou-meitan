use std::fmt::Display;

use dioxus::{events::FormEvent, prelude::*};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;

// -------------------------------------------------------------------------------------------------
/// 何か入力が必要なString
#[derive(Clone)]
pub struct RequiredString(String);

pub struct RequiredError;

impl TryFrom<String> for RequiredString {
    type Error = RequiredError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value != String::default() {
            Ok(RequiredString(value))
        } else {
            Err(RequiredError)
        }
    }
}

impl Display for RequiredString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// -------------------------------------------------------------------------------------------------
// InputType
#[derive(Clone)]
pub enum InputType {
    InputText,
    InputDate,
    TextArea,
    Url,
    InputNum,
}

// -------------------------------------------------------------------------------------------------
// ValidationInput

#[derive(Props)]
pub struct ValidationInputProps<'a, T: TryFrom<String> + ToString + Clone + 'static> {
    // 値の更新時に実行するコールバック
    pub onchange: EventHandler<'a, Option<T>>,
    // バリデーションのエラーメッセージ
    #[props(into)]
    pub error_message: String,
    // ラベルとするコンポーネント
    pub label_component: Element<'a>,
    // Inputのクラス名
    #[props(into)]
    pub class: String,
    // 必須のフィールドであるかどうか
    #[props(default = false)]
    pub required: bool,
    // // エラーメッセージを表示するかどうか
    // pub show_error_message: bool,
    // インプットのタイプ
    pub input_type: InputType,
    /// 最初に与える初期値
    #[props(!optional)]
    pub initial_value: Option<T>,
}

pub fn ValidationInput<'a, T: TryFrom<String> + ToString + Clone + 'static>(
    cx: Scope<'a, ValidationInputProps<'a, T>>,
) -> Element {
    let error_message = use_state(cx, || Some("※必須の項目です".to_string()));
    let input_type = cx.use_hook(|| cx.props.input_type.clone());

    // valueの初期化
    use_effect(cx, (), {
        let mut selector = ".".to_string();
        selector.push_str(&cx.props.class);
        let initial_value = cx.props.initial_value.clone();
        to_owned![error_message];

        |_| async move {
            if let Some(initial_value) = initial_value {
                let input_element = gloo_utils::document()
                    .query_selector(&selector)
                    .unwrap_throw()
                    .unwrap_throw()
                    .unchecked_into::<HtmlInputElement>();
                input_element.set_value(&initial_value.to_string());

                error_message.set(None);
            }
        }
    });

    // String -> Result<T, String>に変換する関数
    let try_into_func = move |s: String| -> Result<T, String> {
        // required
        if cx.props.required {
            let _required_string: RequiredString = s
                .clone()
                .try_into()
                .map_err(|_| "＊必須の項目です．".to_string())?;
        }
        let domain_value: T = s.try_into().map_err(|_| cx.props.error_message.clone())?;
        Ok(domain_value)
    };

    let onchange = move |e: FormEvent| {
        let try_into_res = try_into_func(e.value.clone());
        match try_into_res {
            Ok(domain_value) => {
                error_message.set(None);
                cx.props.onchange.call(Some(domain_value));
            }
            Err(error_s) => {
                error_message.set(Some(error_s));
                cx.props.onchange.call(None);
            }
        }
    };

    let input_component = match input_type {
        InputType::InputText => {
            rsx! {input { class: "{cx.props.class}", r#type: "text", oninput:onchange}}
        }
        InputType::InputDate => {
            rsx! {input { class: "{cx.props.class}", r#type: "date", oninput:onchange}}
        }
        InputType::Url => {
            rsx! {input { class: "{cx.props.class}", r#type: "url", oninput:onchange}}
        }
        InputType::TextArea => {
            rsx! {textarea { class: "{cx.props.class}", oninput:onchange}}
        }
        InputType::InputNum => {
            rsx! {input { class: "{cx.props.class}", r#type: "number", oninput:onchange}}
        }
    };

    cx.render(rsx! {
        label {
            div{ class:"validation-input-label-container",
                &cx.props.label_component,
                error_message.get().as_ref()
                    .map(|message| {
                        rsx! {div { class:"error-message","{message}"}}
                    })
            }
            input_component
        }
    })
}
