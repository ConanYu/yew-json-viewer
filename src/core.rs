use std::ops::Deref;
use std::rc::Rc;
use serde_json::Value;
use yew::prelude::*;
use crate::common::is_collapsable;
use crate::common::value_length;
use crate::interaction::ButtonControlDialogJsonViewer;
use crate::JsonViewerOption;
use crate::css::*;

#[derive(PartialEq, Properties)]
pub struct RenderProps {
    pub value: Rc<Value>,
    pub option: Rc<JsonViewerOption>,
    #[prop_or_default]
    pub father_collapsed: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub is_root: bool,
}

fn collapsed_callback(collapsed: UseStateHandle<Vec<bool>>, index: usize) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        let mut temp = collapsed.deref().clone();
        if index < temp.len() {
            *temp.get_mut(index).unwrap() = !temp.get(index).unwrap();
        }
        collapsed.set(temp);
    })
}

fn collapsed_class(collapsed: bool) -> Option<&'static str> {
    collapsed.then(|| "collapsed")
}

fn is_string_json(value: &Value, use_json5: bool) -> Option<Value> {
    if value.is_string() {
        let s = value.as_str().unwrap();
        if let Ok(value) = if use_json5 {
            json5::from_str::<Value>(s).map_err(|_| ())
        } else {
            serde_json::from_str::<Value>(s).map_err(|_| ())
        } {
            return Some(value);
        }
    }
    None
}

#[function_component(RootRender)]
pub fn root_render(props: &RenderProps) -> Html {
    let RenderProps { value, option, .. } = props;
    let root_collapsable = option.root_collapsable || option.is_force_default_collapse(value_length(value));
    let collapsed = use_state(|| root_collapsable);
    let onclick = Callback::from({
        let collapsed = collapsed.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            collapsed.set(!*collapsed);
        }
    });
    let inner_html = html! {
        <>
            if is_collapsable(value) {
                <a href="" class={classes!(JSON_TOGGLE.as_str(), collapsed_class(*collapsed))} onclick={onclick.clone()}/>
            } else if let Some(value) = is_string_json(value, *option.use_json5.borrow()) {
                <ButtonControlDialogJsonViewer {value} /> 
            }
            <Render value={value.clone()} option={option} father_collapsed={*collapsed} {onclick} is_root={true}/>
        </>
    };
    let style = if option.dialog_index.is_some() { "max-height: 77vh" } else { "" };
    html! {
        <pre class={classes!(JSON_DOCUMENT.as_str())} {style}>
            {inner_html}
        </pre>
    }
}

fn get_collapsed_state(value: Rc<Value>, option: Rc<JsonViewerOption>) -> Vec<bool> {
    match value.deref() {
        Value::Array(arr) => arr.iter().map(|value| value).collect::<Vec<_>>(),
        Value::Object(object) => object.iter().map(|(_, value)| value).collect::<Vec<_>>(),
        _ => vec![],
    }.iter().map(|value| option.collapsable || option.is_force_default_collapse(value_length(value))).collect::<Vec<_>>()
}

#[function_component(Render)]
fn render(props: &RenderProps) -> Html {
    let reverse_father_collapsed = props.onclick.clone();
    let RenderProps { value, option, father_collapsed, ..} = props;
    let length = match value.deref() {
        Value::Array(arr) => arr.len(),
        Value::Object(object) => object.len(),
        _ => 0,
    };
    let collapsed = use_state(|| get_collapsed_state(value.clone(), option.clone()));
    use_effect_with((length).clone(), {
        let collapsed = collapsed.clone();
        let value = value.clone();
        let option = option.clone();
        move |_| {
            collapsed.set(get_collapsed_state(value.clone(), option.clone()));
            || {}
        }
    });
    let mut arr = if props.is_root { vec![] } else { (option.value_to_element)(value) };
    if let Some(addtional_value_to_element) = &option.additional_value_to_element {
        let mut additional_element = (addtional_value_to_element)(value);
        arr.append(&mut additional_element);
    }
    let arr = arr
        .into_iter()
        .enumerate()
        .map(|(index, interaction)| {
            html! {
                <span key={index}>
                    {interaction}
                </span>
            }
        })
        .collect::<Vec<_>>();
    let element = match value.deref() {
        Value::String(s) => {
            let s = serde_json::to_string(s).unwrap();
            html! {
                <span class={classes!(JSON_STRING.as_str())}>{s}</span>
            }
        }
        Value::Number(n) => {
            let n = n.to_string();
            html! { <span class={classes!(JSON_LITERAL.as_str())}>{n}</span> }
        }
        Value::Bool(b) => {
            let b = b.to_string();
            html! { <span class={classes!(JSON_LITERAL.as_str())}>{b}</span> }
        }
        Value::Null => {
            html! { <span class={classes!(JSON_LITERAL.as_str())}>{"null"}</span> }
        }
        Value::Array(arr) => {
            if arr.len() > 0 {
                let mut result = vec![];
                for (index, item) in arr.iter().enumerate() {
                    let mut current_html = vec![];
                    let onclick = collapsed_callback(collapsed.clone(), index);
                    if is_collapsable(item) {
                        current_html.push(html! {
                            <a key="collapse" href="" class={
                                classes!(JSON_TOGGLE.as_str(), 
                                         collapsed_class(collapsed.deref().get(index).unwrap_or(&option.collapsable).clone()))
                            } onclick={onclick.clone()}/>
                        });
                    }
                    current_html.push(html! {
                        <Render key="render" value={Rc::new(item.clone())} option={option} 
                                father_collapsed={collapsed.deref().get(index).unwrap_or(&option.collapsable).clone()}
                                onclick={onclick.clone()}
                        />
                    });
                    result.push(html! {
                        <li key={index}>
                            {current_html}
                            if index != arr.len() - 1 {
                                {","}
                            }
                        </li>
                    });
                }
                let total = arr.len();
                let placeholder = format!("{} {}", total, if total > 1 { "items" } else { "item" });
                html! {
                    <>
                        {"["}
                        if *father_collapsed {
                            <a href="" class={classes!(JSON_PLACEHOLDER.as_str())} onclick={reverse_father_collapsed}>{placeholder}</a>
                        } else {
                            <ol class={classes!(JSON_ARRAY.as_str())}>{result}</ol>
                        }
                        {"]"}
                    </>
                }
            } else {
                html! {
                    <>{"[]"}</>
                }
            }
        }
        Value::Object(object) => {
            let mut key_count = object.len();
            if key_count > 0 {
                let result = object
                    .iter()
                    .enumerate()
                    .map(|(index, (key, value))| {
                        let key_repr = html! {
                            <span class={classes!(JSON_STRING.as_str())}>
                                {serde_json::to_string(key).unwrap()}
                            </span>
                        };
                        let onclick = collapsed_callback(collapsed.clone(), index);
                        let key_repr = if is_collapsable(value) {
                            html! {
                                <a href="" class={
                                    classes!(JSON_TOGGLE.as_str(), 
                                             collapsed_class(collapsed.deref().get(index).unwrap_or(&option.collapsable).clone()))
                                } onclick={onclick.clone()}>{key_repr}</a>
                            }
                        } else {
                            key_repr
                        };
                        key_count -= 1;
                        html! {
                            <li key={index}>
                                {key_repr}
                                {": "}
                                <Render value={Rc::new(value.clone())} option={option} 
                                        father_collapsed={collapsed.deref().get(index).unwrap_or(&option.collapsable).clone()}
                                        {onclick}
                                />
                                if key_count > 0 {
                                    {","}
                                }
                            </li>
                        }
                    })
                    .collect::<Vec<_>>();
                let total = object.len();
                let placeholder = format!("{} {}", total, if total > 1 { "items" } else { "item" });
                html! {
                    <>
                        {"{"}
                        if *father_collapsed {
                            <a href="" class={classes!(JSON_PLACEHOLDER.as_str())}
                                onclick={reverse_father_collapsed}>{placeholder}</a>
                        } else {
                            <ul class={classes!(JSON_DICT.as_str())}>{result}</ul>
                        }
                        {"}"}
                    </>
                }
            } else {
                html! {
                    <>{"{}"}</>
                }
            }
        }
    };
    html! {
        <>
            {arr}
            {element}
        </>
    }
}
