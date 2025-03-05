use once_cell::sync::Lazy;
use stylist::style;

pub static JSON_DOCUMENT: Lazy<String> = Lazy::new(|| {
    let style = style!(
        r#"
        padding: 1em 2em;
        border: 1px solid #aaa;
        margin-top: 0.5em;
        overflow: auto;
        font-size: 14px;
    "#).unwrap();
    style.get_class_name().to_string()
});

pub static JSON_DICT: Lazy<String> = Lazy::new(|| {
    let style = style!(r#"
        list-style-type: none;
        margin: 0 0 0 1px;
        border-left: 1px dotted #ccc;
        padding-left: 2em;
    "#).unwrap();
    style.get_class_name().to_string()
});

pub static JSON_ARRAY: Lazy<String> = Lazy::new(|| {
    let style = style!(r#"
        list-style-type: none;
        margin: 0 0 0 1px;
        border-left: 1px dotted #ccc;
        padding-left: 2em;
    "#).unwrap();
    style.get_class_name().to_string()
});

pub static JSON_STRING: Lazy<String> = Lazy::new(|| {
    let style = style!("color: #0B7500;").unwrap();
    style.get_class_name().to_string()
});

pub static JSON_LITERAL: Lazy<String> = Lazy::new(|| {
    let style = style!(r#"
        color: #1A0166;
    "#).unwrap();
    style.get_class_name().to_string()
});

pub static JSON_TOGGLE: Lazy<String> = Lazy::new(|| {
    let style = style!(r#"
        position: relative;
        color: inherit;
        text-decoration: none;
        &:focus {
            outline: none;
        }
        &:before {
            font-size: 1.1em;
            color: #c0c0c0;
            content: "\25BC"; /* down arrow */
            position: absolute;
            display: inline-block;
            width: 1em;
            text-align: center;
            line-height: 1em;
            left: -1.2em;
        }
        &:hover:before {
            color: #aaa;
        }
        &.collapsed:before {
            transform: rotate(-90deg);
        }
    "#).unwrap();
    style.get_class_name().to_string()
});

pub static JSON_PLACEHOLDER: Lazy<String> = Lazy::new(|| {
    let style = style!(r#"
        color: #aaa;
        padding: 0 1em;
        text-decoration: none;
        &:hover {
            text-decoration: underline;
        }
    "#).unwrap();
    style.get_class_name().to_string()
});

pub static OVERLAY_CSS: Lazy<String> = Lazy::new(|| {
    let style = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 900;
    "#
    )
    .unwrap();
    style.get_class_name().to_string()
});

pub static DIALOG_CSS: Lazy<String> = Lazy::new(|| {
    let style = style!(
        r#"
        background-color: white;
        border-radius: 5px;
        padding: 20px;
        width: 70%;
        max-width: 85vw;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    "#
    )
    .unwrap();
    style.get_class_name().to_string()
});

pub static INTERACTION_BUTTON_CSS: Lazy<String> = Lazy::new(|| {
    let style = style!(
        r#"
        display: inline-block;
        width: 1em;
        height: 1em;
        background-color: #ddd;
        border-radius: 50%;
        vertical-align: middle;
        margin: -3px 5px 0 -3px;
        &:hover {
            background-color: #d0d0d0;
        }
    "#
    )
   .unwrap();
    style.get_class_name().to_string()
});