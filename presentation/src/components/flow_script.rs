use dioxus::prelude::*;
use domain::video::{Original, Video};
use frontend::{commands::video_commands, usecases::video_usecase};

pub fn FlowScript(cx: Scope) -> Element {
    let flow_text_base = "莉光迷站はおりコウの非公式ファンページです。";
    let late_video_state = use_state(cx, || Option::<Video<Original>>::None);

    use_effect(cx, (), {
        to_owned![late_video_state];
        |_| async move {
            let res = {
                let cmd = video_commands::OrderByDateVideosCommand::new(1);
                video_usecase::order_by_date_videos::<Original>(cmd).await
            };

            match res {
                Ok(mut late_videos) => {
                    if let Some(first) = late_videos.first_mut() {
                        late_video_state.set(Some(std::mem::take(first)));
                    }
                }
                Err(e) => {
                    log::error!("{e}");
                }
            }
        }
    });

    cx.render(rsx! {
        div {id: "flow-script-container",
            div {id: "flow-script-inner",
                // "莉光迷站はおりコウの非公式ファンページです。"
                span {"{flow_text_base}"}
                late_video_state.get().as_ref().map(|late_video| {
                    let url_str = late_video.url().to_string();
                    let title = late_video.title();
                    rsx!{
                        span {"おりコウの最新動画"}
                        a {href:"{url_str}", "{title}"}
                        span {"をチェック！"}
                    }
                })
            }
        }
    })
}
