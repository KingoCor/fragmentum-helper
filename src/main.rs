mod components;
mod pages;

use leptos::*;
use leptos_router::*;

use components::nav::Nav;

use pages::under_construction::UnderConstruction;
use pages::race_about_template::RaceAboutTemplate;

fn main() {
    mount_to_body(|| view! {
        <Router>
            <Nav/>
            <main>
                <Routes>
                    <Route path="/" view=UnderConstruction/>
                    <Route path="/create_race" view=UnderConstruction/>
                    <Route path="/resource_calculator" view=UnderConstruction/>
                    <Route path="/detachment_editor" view=UnderConstruction/>
                    <Route path="/race_about_template" view=RaceAboutTemplate/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    })
}
