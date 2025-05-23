use crate::components::challange::components::A;
use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos::prelude::*;
use leptos::Params;
use leptos::*;
use leptos::*;
use leptos::*;
use leptos_meta::*;
use leptos_meta::*;
use leptos_router::hooks::use_params;
use leptos_router::nested_router::Outlet;
use leptos_router::*;
use leptos_router::*;
use leptos_use::{use_interval, UseIntervalReturn};
use serde::{self, Deserialize, Serialize};
use std::cmp::{max, min};

use crate::structs::*;

#[component]
pub fn ChallangeC(challange: Challange) -> impl IntoView {
    let Challange { name, id, done } = challange;
    view! {
            <A  href=format!{"/challange/{}", id}>
            <h1 class="challangec">{name.to_string()} </h1>
            </A>
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

#[server]
pub async fn submit_code(content: String) -> Result<(), ServerFnError> {
    Ok(())
}

#[component]
pub fn ChallangeSite<'a>(data: &'a ChallangeWContent) -> impl IntoView {
    let ChallangeWContent {
        challange: challange,
        content: challange_content,
    } = data;
    let ChallangeContent {
        given: challange_content_given,
    } = challange_content;
    let Challange {
        id: challange_id,
        name: challange_name,
        done: challange_done,
    } = challange;
    //let code_input = RwSignal::new("init".to_string());
    let (code_input, set_code_input) = signal("init".to_string());

    //<ActionFrom action=submit_code>
    //</ActionFrom>

    view! {
	<div class="reset_button">
	    <button>reset</button>
	    </div>

	// <div>
	//    <div>USER TAB</div>
	//    <div>GIVEN TAB</div>
	//    <button>reset</button>
	// </div>

	    <div class="input_field">
	    <input type="text"
        //bind:value=(code_input, set_code_input)
            bind:value=(code_input, set_code_input)
        //value={code_input}
            />
	    </div>
	    <p> "current input:" {code_input} </p>
	    <div >  {
		challange_name.clone()
	    } </div>

	    <div> {
		challange.id

	    } </div>

	    <div> {
		challange_content_given.clone()

	    } </div>

	    <CodeView/>
    }
}

#[component]
pub fn ChallangePageHeader() -> impl IntoView {
    view! {
	<div class="ide">
	    <div class="tabs">
	    <A href="">USER TAB</A>
	    <A href="given">GIVEN TAB</A>
	    </div>
	    <Outlet/>
	    </div>
    }
}

#[component]
pub fn ChallangePage() -> impl IntoView {
    let params = use_params::<ChallangeSiteParams>();

    let id = move || params.get().unwrap().id.unwrap();
    view! {


        <Await
        future =get_challange_site(id())
        let:data
            >
            <ChallangeSite data=data.as_ref().unwrap()/>



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
        future = get_challanges()
        let:data
      >
      {

          data.to_owned().unwrap_or_default().into_iter().map(|m| view! {
      <ChallangeC challange=m/>
          }).collect_view()
     }
      </Await>
      </ul>
    }
}
