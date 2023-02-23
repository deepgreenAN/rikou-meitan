use dioxus::{events::FormEvent, prelude::*};

// -------------------------------------------------------------------------------------------------
/// 何か入力が必要なString
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

// -------------------------------------------------------------------------------------------------
// ValidationInput

#[derive(Props)]
pub struct ValidationInputTextProps<'a, T: TryFrom<String>> {
    // 値の更新時に実行するコールバック
    pub onchange: EventHandler<'a, Option<T>>,
    // バリデーションのエラーメッセージ
    pub error_message: String,
    // 必須のフィールドであるかどうか
    pub required: bool,
    // エラーメッセージを表示するかどうか
    pub show_error_message: bool,
}

pub fn ValidationInputText<'a, T: TryFrom<String>>(
    cx: Scope<'a, ValidationInputTextProps<'a, T>>,
) -> Element {
    let required_error_message = "必須のフィールドです";
    let error_message_state = use_state(cx, || Some(required_error_message.to_string()));

    // String -> Result<T, String>に変換する関数
    let try_into_func = move |s: String| -> Result<T, String> {
        // required
        if cx.props.required {
            let _required_string: RequiredString = s
                .clone()
                .try_into()
                .map_err(|_| required_error_message.to_string())?;
        }
        let domain_value: T = s.try_into().map_err(|_| cx.props.error_message.clone())?;
        Ok(domain_value)
    };

    let onchange = move |e: FormEvent| {
        let try_into_res = try_into_func(e.value.clone());
        match try_into_res {
            Ok(domain_value) => {
                error_message_state.set(None);
                cx.props.onchange.call(Some(domain_value));
            }
            Err(error_message) => {
                error_message_state.set(Some(error_message));
                cx.props.onchange.call(None);
            }
        }
    };

    cx.render(rsx! {
        input { r#type: "text", onchange:onchange}
        cx.props.show_error_message.then_some(())
        .and(
            error_message_state.get().as_ref()
        )
        .map(|message| {
            rsx! {span {"{message}"}}
        })
    })
}
