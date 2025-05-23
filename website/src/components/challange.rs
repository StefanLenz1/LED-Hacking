use crate::components::challange::components::A;
use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::Params;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::*;
use leptos::*;
use leptos::*;
use leptos_meta::*;
use leptos_meta::*;
use leptos_router::hooks::use_params;
use leptos_router::nested_router::Outlet;
use leptos_router::*;
use leptos_router::*;
use leptos_use::{UseIntervalReturn, use_interval};
use serde::{self, Deserialize, Serialize};
use std::cmp::{max, min};
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
        format!("content start {} content end", id),
    ));
}

#[server(GetChallangeSiteHint)]
pub async fn get_challange_site_hint(id: u64) -> Result<String, ServerFnError> {
    Ok(format!("hint for challange number {}", (id)).to_string())
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
pub fn ChallangeSite(data: ChallangeWContent) -> impl IntoView {
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
    let submit_code = ServerAction::<SubmitCode>::new();
    let value = submit_code.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));
    let (count, set_count) = signal(0);

    let reset_controller = ServerAction::<ResetController>::new();
    //let clear_code = Action::new(move |_: &()| async move { async move { reset_controller().await } });

    //<ActionFrom action=submit_code>
    //</ActionFrom>
    //
    //
    //
    let code_input_element: NodeRef<html::Input> = NodeRef::new();
    let on_submit_code = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = code_input_element
            .get()
            .expect("input should be mounted")
            .value();
        //set_code_input.set(value);
        //submit_code.dispatch(SubmitCode { content: value });
    };

    let (code, set_code) = signal("intial code".to_string());
    view! {
    <form on:submit=on_submit_code >
        <div>
        <input type="submit" value="upload"/>
        <textarea prop:value=move | | code_input.get() on:input:target=move |ev| set_code_input.set(ev.target().value()) >{
            code_input
        }</textarea>
        </div>

        <div>
        <ActionForm action=reset_controller>
          <input type="submit" value="reset"/>
        </ActionForm>
        </div>
        //bind:value=(code_input, set_code_input)
        //value={code_input}
    //<div class="reset_button">
    //   <button>reset</button>
    //</div>
       //<button on:click=move |_| {
       //    println!("button clicked");
       //     spawn_local(async {
       //         submit_code("testlkjllakjflöajdflöajkdflöj".to_string()).await;
       //     });}
       //>upload code</button>
       //
    </form>


    //<Form action=clear_code>
     // <button
     // on:click=move |_| reset_count_set.set(2)
     // //{
     //     //clear_code.dispatch(());

     //     //println!("dispatched action");
     // //}
     // > "reset" </button>
     //<p> {move || reset_count.get()} </p>

     //<p>{move || { if clear_code.pending().get() {"pendig..."} else {"not pending"}}}</p>
    //</ActionForm>

    // <div>
    //    <div>USER TAB</div>
    //    <div>GIVEN TAB</div>
    //    <button>reset</button>
    // </div>

        <div class="input_field">
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
pub fn ChallangePageGiven() -> impl IntoView {
    let params = use_params::<ChallangeSiteParams>();
    let id = move || params.get().unwrap().id.unwrap();

    view! {
        <Await
        future =get_challange_site_hint(id())
        let:data
            >
            <ChallangeSiteHint data=data.clone().unwrap()/>
        </Await>
    }
}

#[component]

pub fn ChallangeSiteHint(data: String) -> impl IntoView {
    view! {
        <div> {data.clone()} </div>
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
            <ChallangeSite data=data.clone().unwrap()/>
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
