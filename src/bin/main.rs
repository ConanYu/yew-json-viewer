use std::ops::Deref;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_json_viewer::{css::JSON_DOCUMENT, common::CopyButton, JsonViewer};

#[function_component(App)]
pub fn app() -> Html {
    let data = use_state(|| "".to_string());
    let oninput = {
        let data = data.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<web_sys::EventTarget> = e.target();
            let element = target.and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok());
            data.set(element.unwrap().value());
        })
    };
    
    let raw_text_element = |text: String| {
        html! {
            <>
                <span class="h5">{"Parse Result"}</span>
                if !text.is_empty() {
                    <span style="vertical-align: 0.2em; margin-left: 0.2em;">
                        <CopyButton text={text.clone()}/>
                    </span>
                }
                <pre class={JSON_DOCUMENT.as_str()}>{text}</pre>
            </>
        }
    };

    let element = if data.deref().is_empty() {
        raw_text_element("".to_string())
    } else {
        match serde_json::from_str::<serde_json::Value>(data.deref().as_str()) {
            Ok(value) => html! {
                <>
                    <span class="h5">{"Parse Result"}</span>
                    <span style="vertical-align: 0.2em; margin-left: 0.2em;">
                        <CopyButton text={serde_json::to_string_pretty(&value).unwrap()}/>
                    </span>
                    {JsonViewer::new(value).render()}
                </>
            },
            Err(err) => {
                let msg = format!("Parse error: {}", err.to_string());
                raw_text_element(msg)
            },
        }
    };
    html! {
        <>
            <div class="container-fluid" style="margin-top: 10px;">
                <div class="row">
                    <div class="col-4">
                        <textarea class="form-control" style="resize: none; margin-top: 5px; width: 100%; height: 92vh;" placeholder="Input your JSON" {oninput}/>
                        <div style="margin-top: 12px;">
                            <span style="cursor: pointer" onclick={Callback::from(move |_| {
                                    let url = "https://github.com/conanyu/yew-json-viewer";
                                    gloo::utils::window().open_with_url_and_target(url, "_blank").unwrap();
                                })}>
                                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" class="bi bi-github" viewBox="0 0 16 16">
                                    <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                                </svg>
                            </span>
                            <span style="margin-left: 8px;">
                                {"The entered data will only be parsed locally."}
                            </span>
                        </div>
                    </div>
                    <div class="col-8">
                        {element}
                    </div>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}