pub mod css;
pub mod common;
pub mod core;
pub mod interaction;

use std::cell::RefCell;
use std::rc::Rc;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties)]
pub struct JsonViewerOption {
    pub collapsable: bool,
    pub root_collapsable: bool,
    pub value_to_element: Box<dyn Fn(&Value) -> Vec<VNode>>,
    #[prop_or_default]
    pub additional_value_to_element: Option<Box<dyn Fn(&Value) -> Vec<VNode>>>,
    #[prop_or_default]
    pub force_default_collapse_length_gte: usize,
    #[prop_or_default]
    pub dialog_index: Option<i32>,
    #[prop_or_default]
    pub use_json5: Rc<RefCell<bool>>,
}

impl JsonViewerOption {
    pub fn is_force_default_collapse(&self, length: usize) -> bool {
        self.force_default_collapse_length_gte > 0 && length >= self.force_default_collapse_length_gte
    }
}

impl PartialEq for JsonViewerOption {
    fn eq(&self, other: &Self) -> bool {
        self.collapsable == other.collapsable
        && self.root_collapsable == other.root_collapsable
        && self.force_default_collapse_length_gte == other.force_default_collapse_length_gte
        && self.dialog_index == other.dialog_index
    }
}

impl Default for JsonViewerOption {
    fn default() -> Self {
        let use_json5 = Rc::new(RefCell::new(false));
        Self {
            collapsable: false,
            root_collapsable: false,
            force_default_collapse_length_gte: 100,
            dialog_index: None,
            value_to_element: interaction::default_interaction(use_json5.clone()),
            additional_value_to_element: None,
            use_json5: use_json5.clone(),
        }
    }
}

pub struct JsonViewer {
    value: Value,
    option: JsonViewerOption,
}

impl JsonViewer {
    pub fn new(value: Value) -> Self {
        Self::new_with_option(value, Default::default())
    }

    pub fn new_with_option(value: Value, option: JsonViewerOption) -> Self {
        Self { value, option }
    }

    pub fn render(self) -> Html {
        let option = Rc::new(self.option);
        let value = Rc::new(self.value);
        html! {
            <core::RootRender value={value} option={option}/>
        }
    }
}

#[wasm_bindgen]
pub struct JsonViewRenderOption {
    pub collapsable: Option<bool>,
    pub root_collapsable: Option<bool>,
    pub force_default_collapse_length_gte: Option<usize>,
    pub use_json5: Option<bool>,
}

#[wasm_bindgen]
impl JsonViewRenderOption {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            collapsable: None,
            root_collapsable: None,
            force_default_collapse_length_gte: None,
            use_json5: None,
        }
    }
}

#[wasm_bindgen]
pub fn json_view_render(id: &str, value: &str, option: JsonViewRenderOption) {
    let value: Value = if option.use_json5.is_some_and(|b| b) {
        json5::from_str(value).expect(format!("JSON parse error: {}", value).as_str())
    } else {
        serde_json::from_str(value).expect(format!("JSON parse error: {}", value).as_str())
    };
    let element = gloo::utils::document().get_element_by_id(id).expect(format!("element({}) not found", id).as_str());
    let mut renderer_option = JsonViewerOption::default();
    if let Some(collapsable) = option.collapsable {
        renderer_option.collapsable = collapsable;
    }
    if let Some(root_collapsable) = option.root_collapsable {
        renderer_option.root_collapsable = root_collapsable;
    }
    if let Some(force_default_collapse_length_gte) = option.force_default_collapse_length_gte {
        renderer_option.force_default_collapse_length_gte = force_default_collapse_length_gte;
    }
    if let Some(use_json5) = option.use_json5 {
        *renderer_option.use_json5.borrow_mut() = use_json5;
    }
    let renderer = yew::Renderer::<core::RootRender>::with_root_and_props(element, core::RenderProps {
        value: Rc::new(value),
        option: Rc::new(renderer_option),
        father_collapsed: Default::default(),
        onclick: Default::default(),
        is_root: Default::default(),
    });
    renderer.render();
}