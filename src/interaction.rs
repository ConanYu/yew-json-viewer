use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{set_body_overflow_style, value_length, CopyButton},
    css::*,
    JsonViewer, JsonViewerOption,
};
use gloo::utils::{document, window};
use serde_json::Value;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;

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
            window()
                .add_event_listener_with_callback(
                    "keydown",
                    keydown_handler.as_ref().unchecked_ref(),
                )
                .unwrap();
            move || {
                if index == 0 {
                    set_body_overflow_style("auto");
                }
                window()
                    .remove_event_listener_with_callback(
                        "keydown",
                        keydown_handler.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        }
    });
    let onclick_dialog = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });
    let overlay_css = if index == &0 {
        ""
    } else {
        "background-color: rgba(0, 0, 0, 0.4);"
    };
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
pub struct ButtonControlDialogJsonViewerProps {
    pub value: Value,
}

#[function_component(ButtonControlDialogJsonViewer)]
pub fn button_control_dialog_json_viewer(props: &ButtonControlDialogJsonViewerProps) -> Html {
    let ButtonControlDialogJsonViewerProps { value } = props;
    let open = use_state(|| false);
    let mut option: JsonViewerOption = Default::default();
    let index = document()
        .get_elements_by_class_name("yew_json_viewer_dialog")
        .length() as i32;
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
            <span style="cursor: pointer;" {onclick}>
                <div class={INTERACTION_BUTTON_CSS.as_str()}/>
            </span>
            if *open {
                <Dialog {set_close} {index}>
                    <span class="h5">{"JSON"}</span>
                    <span style="vertical-align: 0.2em; margin-left: 0.2em;">
                        <CopyButton text={value.to_string()}/>
                    </span>
                    {viewer.render()}
                </Dialog>
            }
        </>
    }
}

#[derive(PartialEq, Properties)]
struct LongTextViewerProps {
    text: String,
}

#[function_component(LongTextViewer)]
fn long_text_viewer(props: &LongTextViewerProps) -> Html {
    let open = use_state(|| false);
    let LongTextViewerProps { text } = props;
    let onclick = Callback::from({
        let open = open.clone();
        move |_| {
            open.set(true);
        }
    });
    let set_close = {
        let open = open.clone();
        Callback::from(move |_| {
            open.set(false);
        })
    };
    html! {
        <>
            <span style="cursor: pointer;" {onclick}>
                <div class={INTERACTION_BUTTON_CSS.as_str()}/>
            </span>
            if *open {
                <Dialog {set_close}>
                    <span class="h5">{"Long Text"}</span>
                    <span style="vertical-align: 0.2em; margin-left: 0.2em;">
                        <CopyButton text={text.to_string()}/>
                    </span>
                    <pre class={JSON_DOCUMENT.as_str()} style="white-space: pre-wrap; max-height: 80vh;">
                        {text}
                    </pre>
                </Dialog>
            }
        </>
    }
}

#[wasm_bindgen(inline_js = r#"
export function with_global_javascript_interaction() {
    return !!window.__yew_json_viewer__interaction;
}
export function do_global_javascript_interaction(json_string) {
    return window.__yew_json_viewer__interaction(json_string);
}
"#)]
extern "C" {
    pub fn with_global_javascript_interaction() -> bool;
    pub fn do_global_javascript_interaction(json_string: String) -> Option<js_sys::Function>;
}

pub fn default_interaction(use_json5: Rc<RefCell<bool>>) -> Box<dyn Fn(&Value) -> Vec<VNode>> {
    Box::new(
        move |arg: &Value| -> Vec<VNode> {
            if with_global_javascript_interaction() {
                if let Some(func) = do_global_javascript_interaction(arg.to_string()) {
                    let onclick: Callback<MouseEvent> = Callback::from(move |_: MouseEvent| {
                        func.call0(&JsValue::NULL).unwrap();
                    });
                    return vec![html! {
                        <span style="cursor: pointer;" {onclick}>
                            <div class={INTERACTION_BUTTON_CSS.as_str()}/>
                        </span>
                    }];
                }
            }
            let mut result = vec![];
            match arg {
                Value::String(s) => {
                    if let Ok(url) = url::Url::parse(s.as_str()) {
                        if ["http", "https", "ftp", "ftps"].contains(&url.scheme()) {
                            let onclick = Callback::from(move |_: MouseEvent| {
                                window()
                                    .open_with_url_and_target(url.as_str(), "_blank")
                                    .unwrap();
                            });
                            result.push(html! {
                                <span style="cursor: pointer;" {onclick}>
                                    <div class={INTERACTION_BUTTON_CSS.as_str()}/>
                                </span>
                            });
                        }
                    } else {
                        let value = if *use_json5.borrow() {
                            json5::from_str::<Value>(s.as_str()).map_err(|e| e.to_string())
                        } else {
                            serde_json::from_str::<Value>(s.as_str()).map_err(|e| e.to_string())
                        };
                        if let Ok(value) = value {
                            match value {
                                Value::Object(_) | Value::Array(_) | Value::String(_) => {
                                    result.push(html! {
                                        <ButtonControlDialogJsonViewer value={value.clone()}/>
                                    });
                                }
                                _ => {}
                            }
                        } else if s.len() > 100 {
                            result.push(html! {
                                <LongTextViewer text={s.clone()}/>
                            });
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
    )
}
