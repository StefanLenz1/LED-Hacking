use crate::components::challange::components::A;
use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
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
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

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
        vec![format!("content start {} content end", id)],
        HashMap::from([("code file one".to_string(), "code file two".to_string())]),
    ));
}

#[server(GetChallangeSiteHint)]
pub async fn get_challange_site_hints(id: u64) -> Result<Vec<String>, ServerFnError> {
    Ok(vec![
        format!("A hint for challange number {}", (id)).to_string(),
        format!("B hint for challange number {}", (id)).to_string(),
        format!("C hint for challange number {}", (id)).to_string(),
        format!("D hint for challange number {}", (id)).to_string(),
        format!("E hint for challange number {}", (id)).to_string(),
    ])
}

#[server(SubmitCode)]
pub async fn submit_code(content: String) -> Result<(), ServerFnError> {
    let mut file = File::create("/tmp/foo.txt")?;
    //file.write_all(content)?;
    println!("server function called");
    write!(file, "{}", content);
    Ok(())
}

#[server(ResetController)]
pub async fn reset_controller() -> Result<(), ServerFnError> {
    println!("resetting controller");
    Ok(())
}

#[component]
pub fn ChallangeSite<'a>(data: &'a ChallangeWContent) -> impl IntoView {
    let ChallangeWContent {
        challange: challange,
        content: challange_content,
    } = data;
    let ChallangeContent {
        tips: tips,
        code_context: code_context,
    } = challange_content;
    let Challange {
        id: challange_id,
        name: challange_name,
        done: challange_done,
    } = challange;
    //let code_input = RwSignal::new("init".to_string());
    let submit_code = ServerAction::<SubmitCode>::new();
    let value = submit_code.value();

    let reset_controller = ServerAction::<ResetController>::new();
    //let clear_code = Action::new(move |_: &()| async move { async move { reset_controller().await } });

    //<ActionFrom action=submit_code>
    //</ActionFrom>
    //

    view! {

    <ActionForm action=submit_code attr:class="from">
        <div class="buttons">
        <input type="submit" value="upload"/>
        <ActionForm action=reset_controller>
            <input type="submit" value="reset"/>
        </ActionForm>
        </div>
        <div class="text_container">
            <textarea name="content" placeholder="Input your Code here"></textarea>
        </div>
    </ActionForm>
        //<div class="log_window">
        //</div>
    }
}

#[component]
pub fn ChallangePageHeader() -> impl IntoView {
    let params = use_params::<ChallangeSiteParams>();
    let id = move || params.get().unwrap().id.unwrap();
    view! {
    <div class="ide">

        <div class="padding">
        <Await
        future =get_challange_site(id())
        let:data
            >
            <div class="challange_name"> {data.clone().unwrap().challange.name}</div>
        </Await>
            <div class="tabs">
            <A href="">USER TAB</A>
            <A href="given">GIVEN TAB</A>
            </div>
            <Outlet/>
            </div>
        </div>
    }
}
#[component]
pub fn ChallangePageGiven() -> impl IntoView {
    let params = use_params::<ChallangeSiteParams>();
    let id = move || params.get().unwrap().id.unwrap();

    view! {
        <Await
        future =get_challange_site_hints(id())
        let:data
            >
            <ChallangeSiteHint data=data.as_ref().unwrap()/>
        </Await>
    }
}

#[component]

pub fn ChallangeSiteHint<'a>(data: &'a Vec<String>) -> impl IntoView {
    view! {
        {data.clone().iter().enumerate().map(|(idx, hint)| {
            let hint_with_br = format!("Hint {}: {}", idx + 1, hint)
                .replace("\n", "<br>");
            view! {
                <div inner_html=hint_with_br></div>
            }
        }).collect_view()}
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
