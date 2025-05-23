use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_use::{use_interval, UseIntervalReturn};
use serde::{self, Deserialize, Serialize};
use std::cmp::{max, min};

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
    use leptos_router::path;
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/esp-lsd-hacking.css"/>

        // sets the document title
        <Title text="ESP LSD HACKING"/>
        <head/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback =|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        } >
                    <Route path=path!("/") view=HomePage/>
                    //<ParentRoute path=path!("/challange") view =ChallangeList>
                    <ParentRoute path=path!("/challange") view=||view!{ <Outlet/>}>
                       <ParentRoute path=path!(":id") view=ChallangePageHeader>
                         <Route path=path!("") view=ChallangePage/>
                         <Route path=path!("given") view=ChallangePageGiven />
                       </ParentRoute>
                       <Route path=path!("") view=|| view!{} />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
    <div class="main_box">
     <div class="responsive-size">

      <div class="title_box">
         <h1 class="heading"> ESP LSD HACKING</h1>
      </div>
      <ChallangeList/>
      //<Challange name="test"/>
     </div>
    </div>
     }
}
