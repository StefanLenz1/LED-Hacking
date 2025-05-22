use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::{use_interval, UseIntervalReturn};
use serde::{self, Deserialize, Serialize};
use std::cmp::{max, min};

type KEY = usize;

#[cfg(feature = "ssr")]
pub mod ssr {
    //use crate::app::{MachineS, MachineState, KEY};
    use itertools::Itertools;
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{Read, Write};
    use std::sync::LazyLock;
    static PATH: LazyLock<String> =
        LazyLock::new(|| std::env::var("LEPTOS_C_CODE_TMP").unwrap_or("/tmp".to_string()));

    #[derive(Serialize, Deserialize)]
    struct EnteredCode {
        content: String,
    }
    impl EnteredCode {
        pub fn new(content: impl ToString) -> Self {
            Self {
                content: content.to_string(),
            }
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/esp-lsd-hacking.css"/>

        // sets the document title
        <Title text="ESP LSD HACKING"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                </Routes>
                <Routes>
                    <Route path="/challange" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(getChallanges)]
pub async fn get_challanges() -> Result<Vec<Challange>, ServerFnError> {
    return Ok(vec![
        Challange::new("challange a", 1, "nothing given"),
        Challange::new("challange a", 1, "nothing given"),
    ]);
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
    <div class="centered">
     <div class="responsive-size">

      <div class="container">
         <h1 class="heading"> ESP LSD HACKING</h1>
      </div>
      //<Challange name="test"/>
      <ul class="challanges" role="list">

      <Await
        future =|| get_challanges()
        let:data
      >
      {
          data.to_owned().unwrap_or_default().into_iter().enumerate().map(|(idx, m)| view! {
      <ChallangeC name={m.name}/>
          }).collect_view()
     }
      </Await>
      </ul>
     </div>
    </div>
     }
}
