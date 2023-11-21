use leptos::*;
use crate::GITHUB_ROOT;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav>
            <a href=format!("/{}/", GITHUB_ROOT)>
                <h1>"Fragmentum"</h1>
                <h2>"помощник"</h2>
            </a>
            <ul>
                <li><a href=format!("/{}/create_race", GITHUB_ROOT)><button>{ "Создaть государство" }</button></a></li>
                <li><a href=format!("/{}/resource_calculator", GITHUB_ROOT)><button>{ "Калькулятор ресурсов" }</button></a></li>
                <li><a href=format!("/{}/create_detachment", GITHUB_ROOT)><button>{ "Конструктор отрядов" }</button></a></li>
                <li><a href=format!("/{}/race_about_template", GITHUB_ROOT)><button>{ "Шаблон \"Ваша страна как\"" }</button></a></li>
            </ul>
        </nav>
    }
}
