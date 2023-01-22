use std::collections::HashMap;
use yew::html;
use crate::plugins::{CommandItems, DropDownItem};

pub fn emojies() -> CommandItems {
    vec![
        DropDownItem {
            value: html! {"😂"},
            text: "this is a bold text".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {"😍"},
            text: "this is a bold text".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
        DropDownItem {
            value: html! {"😎"},
            text: "this is a bold text".to_string(),
            tag: Some("b".to_string()),
            attrs: HashMap::new(),
        },
    ]
}
