use gloo::utils::{document, window};
use serde_json::Value;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use crate::{common::{set_body_overflow_style, value_length, CopyButton}, css::*, JsonViewer, JsonViewerOption};

#[derive(PartialEq, Properties)]
struct DialogProps {
    #[prop_or_default]
    pub set_close: Callback<MouseEvent>,
    #[prop_or(0)]
    pub index: i32,
    pub children: Html,
}

#[function_component(Dialog)]
fn dialog(props: &DialogProps) -> Html {
    let DialogProps {
        set_close,
        index,
        children,
    } = props;
    use_effect_with((), {
        let index = index.clone();
        let set_close = set_close.clone();
        move |_| {
            if index == 0 {
                set_body_overflow_style("hidden");
            };
            let keydown_handler = move |event: KeyboardEvent| {
                if event.key() == "Escape" {
                    let mut max_index = 0;
                    let elements = document().get_elements_by_class_name("yew_json_viewer_dialog");
                    let length = elements.length();
                    for element in 0..length {
                        let element = elements.item(element).unwrap();
                        let index = element.get_attribute("data-index").unwrap();
                        max_index = index.parse::<i32>().unwrap().max(max_index);
                    }
                    if max_index == index {
                        set_close.emit(MouseEvent::new("click").unwrap());
                    }
                }
            };
            let keydown_handler = Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(keydown_handler));
            window().add_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref()).unwrap();
            move || {
                if index == 0 {
                    set_body_overflow_style("auto");
                }
                window().remove_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref()).unwrap();
            }
        }
    });
    let onclick_dialog = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });
    let overlay_css = if index == &0 { "" } else { "background-color: rgba(0, 0, 0, 0.4);" };
    html! {
        <div class={classes!(OVERLAY_CSS.as_str(), "yew_json_viewer_dialog")} style={overlay_css}
             data-index={index.to_string()} onclick={set_close.clone()}>
            <div class={classes!(DIALOG_CSS.as_str())} onclick={onclick_dialog}>
                {children.clone()}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ButtonControlDialogJsonViewerProps {
    value: Value,
}

#[function_component(ButtonControlDialogJsonViewer)]
fn button_control_dialog_json_viewer(props: &ButtonControlDialogJsonViewerProps) -> Html {
    let ButtonControlDialogJsonViewerProps { value } = props;
    let open = use_state(|| false);
    let mut option: JsonViewerOption = Default::default();
    let index = document().get_elements_by_class_name("yew_json_viewer_dialog").length() as i32;
    // gloo::console::console_dbg!("index: {}, value: {}", index, value.to_string().as_str());
    option.dialog_index = Some(index);
    let viewer = JsonViewer::new_with_option(value.clone(), option);
    let set_close = {
        let open = open.clone();
        Callback::from(move |_| {
            open.set(false);
        })
    };
    let onclick = Callback::from({
        let open = open.clone();
        move |_| {
            open.set(true);
        }
    });
    html! {
        <>
            <span style="cursor: pointer; margin-right: 2px;" {onclick}>
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-filetype-json" viewBox="0 0 16 16">
                    <path fill-rule="evenodd" d="M14 4.5V11h-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5ZM4.151 15.29a1.176 1.176 0 0 1-.111-.449h.764a.578.578 0 0 0 .255.384c.07.049.154.087.25.114.095.028.201.041.319.041.164 0 .301-.023.413-.07a.559.559 0 0 0 .255-.193.507.507 0 0 0 .084-.29.387.387 0 0 0-.152-.326c-.101-.08-.256-.144-.463-.193l-.618-.143a1.72 1.72 0 0 1-.539-.214 1.001 1.001 0 0 1-.352-.367 1.068 1.068 0 0 1-.123-.524c0-.244.064-.457.19-.639.128-.181.304-.322.528-.422.225-.1.484-.149.777-.149.304 0 .564.05.779.152.217.102.384.239.5.41.12.17.186.359.2.566h-.75a.56.56 0 0 0-.12-.258.624.624 0 0 0-.246-.181.923.923 0 0 0-.37-.068c-.216 0-.387.05-.512.152a.472.472 0 0 0-.185.384c0 .121.048.22.144.3a.97.97 0 0 0 .404.175l.621.143c.217.05.406.12.566.211a1 1 0 0 1 .375.358c.09.148.135.335.135.56 0 .247-.063.466-.188.656a1.216 1.216 0 0 1-.539.439c-.234.105-.52.158-.858.158-.254 0-.476-.03-.665-.09a1.404 1.404 0 0 1-.478-.252 1.13 1.13 0 0 1-.29-.375Zm-3.104-.033a1.32 1.32 0 0 1-.082-.466h.764a.576.576 0 0 0 .074.27.499.499 0 0 0 .454.246c.19 0 .33-.055.422-.164.091-.11.137-.265.137-.466v-2.745h.791v2.725c0 .44-.119.774-.357 1.005-.237.23-.565.345-.985.345a1.59 1.59 0 0 1-.568-.094 1.145 1.145 0 0 1-.407-.266 1.14 1.14 0 0 1-.243-.39Zm9.091-1.585v.522c0 .256-.039.47-.117.641a.862.862 0 0 1-.322.387.877.877 0 0 1-.47.126.883.883 0 0 1-.47-.126.87.87 0 0 1-.32-.387 1.55 1.55 0 0 1-.117-.641v-.522c0-.258.039-.471.117-.641a.87.87 0 0 1 .32-.387.868.868 0 0 1 .47-.129c.177 0 .333.043.47.129a.862.862 0 0 1 .322.387c.078.17.117.383.117.641Zm.803.519v-.513c0-.377-.069-.701-.205-.973a1.46 1.46 0 0 0-.59-.63c-.253-.146-.559-.22-.916-.22-.356 0-.662.074-.92.22a1.441 1.441 0 0 0-.589.628c-.137.271-.205.596-.205.975v.513c0 .375.068.699.205.973.137.271.333.48.589.626.258.145.564.217.92.217.357 0 .663-.072.917-.217.256-.146.452-.355.589-.626.136-.274.205-.598.205-.973Zm1.29-.935v2.675h-.746v-3.999h.662l1.752 2.66h.032v-2.66h.75v4h-.656l-1.761-2.676h-.032Z"/>
                </svg>
            </span>
            if *open {
                <Dialog {set_close} {index}>
                    <span class="h5">{"JSON"}</span>
                    <span style="vertical-align: 0.2em;">
                        <CopyButton text={value.to_string()}/>
                    </span>
                    {viewer.render()}
                </Dialog>
            }
        </>
    }
}

pub fn default_interaction(arg: &Value) -> Vec<VNode> {
    let mut result = vec![];
    match arg {
        Value::String(s) => {
            if let Ok(url) = url::Url::parse(s.as_str()) {
                if ["http", "https", "ftp", "ftps"].contains(&url.scheme()) {
                    let onclick = Callback::from(move |_: MouseEvent| {
                        window().open_with_url_and_target(url.as_str(), "_blank").unwrap();
                    });
                    result.push(html! {
                        <span style="cursor: pointer; margin-right: 2px;" {onclick}>
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-link-45deg" viewBox="0 0 16 16">
                                <path d="M4.715 6.542 3.343 7.914a3 3 0 1 0 4.243 4.243l1.828-1.829A3 3 0 0 0 8.586 5.5L8 6.086a1.002 1.002 0 0 0-.154.199 2 2 0 0 1 .861 3.337L6.88 11.45a2 2 0 1 1-2.83-2.83l.793-.792a4.018 4.018 0 0 1-.128-1.287z"/>
                                <path d="M6.586 4.672A3 3 0 0 0 7.414 9.5l.775-.776a2 2 0 0 1-.896-3.346L9.12 3.55a2 2 0 1 1 2.83 2.83l-.793.792c.112.42.155.855.128 1.287l1.372-1.372a3 3 0 1 0-4.243-4.243L6.586 4.672z"/>
                            </svg>
                        </span>
                    });
                }
            } else if let Ok(value) = serde_json::from_str::<Value>(s.as_str()) {
                match value {
                    Value::Object(_) | Value::Array(_) | Value::String(_) => {
                        result.push(html! {
                            <ButtonControlDialogJsonViewer value={value.clone()}/>
                        });
                    }
                    _ => {}
                }
            }
        }
        Value::Array(_) | Value::Object(_) => {
            if value_length(arg) > 0 {
                result.push(html! {
                    <ButtonControlDialogJsonViewer value={arg.clone()}/>
                });
            }
        }
        _ => {}
    };
    result
}