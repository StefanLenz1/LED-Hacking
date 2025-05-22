use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos::*;
use leptos::*;
use leptos_meta::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::*;
use leptos_use::{use_interval, UseIntervalReturn};
use serde::{self, Deserialize, Serialize};
use std::cmp::{max, min};
#[component]
pub fn ChallangeC(name: impl ToString) -> impl IntoView {
    view! {
             <h1 class="challangec"> {name.to_string()}</h1>
    }
}

impl Challange {
    pub fn new(name: impl ToString, id: u64, content_given: impl ToString) -> Self {
        return Self {
            name: name.to_string(),
            id: id,
            content: ChallangeContent {
                given: content_given.to_string(),
            },
        };
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChallangeContent {
    pub given: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Challange {
    pub name: String,
    pub id: u64,
    pub content: ChallangeContent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChallangeSubmit {
    pub content: ChallangeContent,
    pub id: u64,
}
