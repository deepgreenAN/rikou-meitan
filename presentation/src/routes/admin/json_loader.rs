use dioxus::prelude::*;
use gloo_events::EventListener;
use js_sys::Array;
use serde::{de::DeserializeOwned, Serialize};
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{Blob, BlobPropertyBag, FileReader, HtmlAnchorElement, HtmlInputElement, Url};

#[derive(Props)]
pub struct JsonLoaderProps<T> {
    #[props(into)]
    pub title: String,
    #[props(into)]
    pub id: String,
    pub on_upload: Rc<dyn Fn(Vec<T>)>,
    pub make_json_source: Rc<dyn Fn() -> Vec<T>>,
}

pub fn JsonLoader<T>(cx: Scope<JsonLoaderProps<T>>) -> Element
where
    T: Serialize + DeserializeOwned + 'static,
{
    let error_message = use_state(cx, || Option::<String>::None);

    let download_json = move |_| {
        let anchor = gloo_utils::document()
            .create_element("a")
            .unwrap_throw()
            .unchecked_into::<HtmlAnchorElement>();
        anchor.set_download(&format!("{}.json", cx.props.title));

        let content = (cx.props.make_json_source)();

        let mut blob_option = BlobPropertyBag::new();
        blob_option.type_("application/json");

        let blob = Blob::new_with_str_sequence_and_options(
            &Array::of1(&JsValue::from_str(
                serde_json::to_string(&content)
                    .expect("Json Serialize failed")
                    .as_str(),
            )),
            &blob_option,
        )
        .unwrap_throw();

        let object_url = Url::create_object_url_with_blob(&blob).unwrap_throw();
        anchor.set_href(&object_url);

        anchor.click();

        Url::revoke_object_url(&object_url).unwrap_throw();
    };

    let upload_json = move |_| {
        let input = gloo_utils::document()
            .get_element_by_id(&format!("upload-{}", cx.props.id))
            .unwrap_throw()
            .unchecked_into::<HtmlInputElement>();

        let file = input.files().unwrap_throw().item(0).unwrap_throw();
        let file_reader = Rc::new(FileReader::new().unwrap_throw());
        EventListener::once(&file_reader, "load", {
            to_owned![error_message];
            let on_upload = Rc::clone(&cx.props.on_upload);
            let file_reader = Rc::clone(&file_reader);
            move |_| {
                let result_string = file_reader
                    .result()
                    .unwrap_throw()
                    .as_string()
                    .unwrap_throw();

                let content_res = serde_json::from_str::<Vec<T>>(result_string.as_str());
                match content_res {
                    Ok(content) => {
                        error_message.set(None);
                        on_upload(content);
                    }
                    Err(_) => error_message.set(Some("無効なファイルです".to_string())),
                }
            }
        })
        .forget();

        file_reader.read_as_text(&file).unwrap_throw();
    };

    cx.render(rsx! {
        div{ class: "json-loader-container",
            div {class: "json-loader-caption", "{cx.props.title}"}
            button {class: "json-download-button", onclick: download_json, "jsonファイルをダウンロード"}
            div {class: "json-upload-container",
                input {r#type: "file", id: "upload-{cx.props.id}", onchange: upload_json,"jsonファイルをアップロード"}
            }
            error_message.get().as_ref().map(|error_message|{
                rsx!{
                    div {class: "json-load-error", "{error_message}"}
                }
            })
        }
    })
}
