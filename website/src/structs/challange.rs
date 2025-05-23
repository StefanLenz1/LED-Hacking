use crate::components::*;

use crate::error_template::{AppError, ErrorTemplate};
use leptos::Params;
use std::collections::HashMap;
use leptos::*;
use leptos::*;
use leptos::*;
use leptos_meta::*;
use leptos_meta::*;
use leptos_router::params::Params;
use leptos_router::*;
use leptos_router::*;
use leptos_use::{use_interval, UseIntervalReturn};
use serde::{self, Deserialize, Serialize};
use std::cmp::{max, min};
impl Challange {
    pub fn new(name: impl ToString, id: u64, done: bool) -> Self {
        Self {
            name: name.to_string(),
            id: id,
            done: done,
        }
    }
}
#[derive(Serialize, Deserialize, Params, PartialEq, Clone)]
pub struct ChallangeSiteParams {
    pub id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChallangeWContent {
    pub challange: Challange,
    pub content: ChallangeContent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChallangeContent {
    pub tips: Vec<String>,
    pub code_context: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Challange {
    pub name: String,
    pub id: u64,
    pub done: bool,
    //pub content: ChallangeContent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChallangeSubmit {
    pub content: ChallangeContent,
    pub id: u64,
}

impl ChallangeWContent {
    pub fn new(name: impl ToString, id: u64, content_given: Vec<String>, code_context: HashMap<String, String>) -> Self {
        Self {
            challange: Challange {
                name: name.to_string(),
                id: id,
                done: false,
            },
            content: ChallangeContent {
                tips: content_given,
                code_context: code_context,
            },
        }
    }
}
