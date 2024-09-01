use std::ops::Deref;
use serde_json::Value;
use gloo::utils::window;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(inline_js = r#"
export function copy_text(text) {
    navigator.clipboard.writeText(text).then(() => {});
}
export function set_body_overflow_style(overflow) {
    document.body.style.overflow = overflow;
}
"#)]
extern "C" {
    pub fn copy_text(text: &str);
    pub fn set_body_overflow_style(overflow: &str);
}

#[derive(Properties, PartialEq)]
pub struct CopyButtonProps {
    pub text: String,
}

#[function_component(CopyButton)]
pub fn copy_button(props: &CopyButtonProps) -> Html {
    let checked = use_state(|| false);
    let callback = use_state({
        let checked = checked.clone();
        move || {
            Closure::wrap(Box::new({
                move || {
                    checked.set(false);
                }
            }) as Box<dyn Fn()>)
        }
    });
    let onclick = Callback::from({
        let text = props.text.clone();
        let checked = checked.clone();
        move |_| {
            if *checked {
                return;
            }
            checked.set(true);
            copy_text(text.as_str());
            window().set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.deref().as_ref().unchecked_ref(),
                1000,
            ).unwrap();
        }
    });
    let span_style = format!(
        "margin-left: 4px; {}",
        if *checked { "" } else { "cursor: pointer;" }
    );
    html! {
        <span style={span_style} {onclick}>
            if *checked {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-clipboard-check" viewBox="0 0 16 16">
                    <path fill-rule="evenodd" d="M10.854 7.146a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708 0l-1.5-1.5a.5.5 0 1 1 .708-.708L7.5 9.793l2.646-2.647a.5.5 0 0 1 .708 0z"/>
                    <path d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1v-1z"/>
                    <path d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5h3zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3z"/>
                </svg>
            } else {
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-clipboard" viewBox="0 0 16 16">
                    <path d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1v-1z"/>
                    <path d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5h3zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3z"/>
                </svg>
            }
        </span>
    }
}

pub fn is_collapsable(arg: &Value) -> bool {
    value_length(arg) > 0
}

pub fn value_length(arg: &Value) -> usize {
    match arg {
        Value::Object(map) => map.len(),
        Value::Array(arr) => arr.len(),
        _ => 0,
    }
}
