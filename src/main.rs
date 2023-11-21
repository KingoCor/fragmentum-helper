mod components;
mod pages;
mod aspects;
mod structures;
mod resources;
mod state;
mod detachment;

use leptos::*;
use leptos_router::*;

use components::nav::Nav;

use pages::create_detachment::CreateDetachment;
use pages::create_race::CreateRace;
use pages::under_construction::UnderConstruction;
use pages::race_about_template::RaceAboutTemplate;

const GITHUB_ROOT: &str = "fragmentum-helper";

fn main() {
    mount_to_body(|| view! {
        <Router>
            <Nav/>
            <main>
                <Routes>
                    <Route path=format!("/{}/", GITHUB_ROOT) view=UnderConstruction/>
                    <Route path=format!("/{}/create_race", GITHUB_ROOT) view=CreateRace/>
                    <Route path=format!("/{}/resource_calculator", GITHUB_ROOT) view=UnderConstruction/>
                    <Route path=format!("/{}/create_detachment", GITHUB_ROOT) view=CreateDetachment/>
                    <Route path=format!("/{}/race_about_template", GITHUB_ROOT) view=RaceAboutTemplate/>
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    })
}
