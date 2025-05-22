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

use crate::structs::*;

#[component]
pub fn ChallangeC(name: impl ToString) -> impl IntoView {
    view! {
             <h1 class="challangec"> {name.to_string()}</h1>
    }
}

#[server(GetChallangeSite)]
pub async fn get_challange_site(id: u64) -> Result<ChallangeWContent, ServerFnError> {
    return Ok(ChallangeWContent::new(
        id.to_string(),
        id,
        format!("{}", id),
    ));
}

#[component]
pub fn ChallangeSite() -> impl IntoView {
    view! {
        <Await
        future =|| get_challange_site(1)
        let:data
            >
            <div> {data.to_owned().unwrap().challange.id} </div>
            <CodeView/>
        </Await>
    }
}

#[server(getChallanges)]
pub async fn get_challanges() -> Result<Vec<Challange>, ServerFnError> {
    return Ok(vec![
        Challange::new("challange a", 1, false),
        Challange::new("challange a", 1, false),
    ]);
}

#[component]
pub fn ChallangeList() -> impl IntoView {
    view! {

      <ul class="challanges" role="list">

      <Await
        future =|| get_challanges()
        let:data
      >
      {

          data.to_owned().unwrap_or_default().into_iter().map(|m| view! {
      <ChallangeC name={m.name}/>
          }).collect_view()
     }
      </Await>
      </ul>
    }
}
