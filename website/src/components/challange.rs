use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::Params;
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
        format!("challange number {}", (id)),
        id,
        format!("content start {} content end", id),
    ));
}

#[component]
pub fn ChallangeSite() -> impl IntoView {
    let params = use_params::<ChallangeSiteParams>();

    let id = move || params.get().unwrap().id.unwrap();
    view! {


        <Await
        future =move || get_challange_site(id())
        let:data

            >

            {
                let ChallangeWContent{challange: challange, content: challange_content} = data.as_ref().unwrap();
                let ChallangeContent{given: challange_content_given} = challange_content;
                let Challange{id: challange_id , name: challange_name, done: challange_done} = challange;



                view!{
                      <div > {

                          challange_name
                      } </div>

                      <div> {

                          challange.id

                      } </div>

                      <div> {

                          challange_content_given

                      } </div>
                   }
             }

            <CodeView/>
        </Await>
    }
}

#[server(getChallanges)]
pub async fn get_challanges() -> Result<Vec<Challange>, ServerFnError> {
    return Ok(vec![
        Challange::new("challange 1", 1, false),
        Challange::new("challange 2", 2, false),
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
