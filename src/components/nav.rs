use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav>
            <a href="/">
                <h1>"Fragmentum"</h1>
                <h2>"помощник"</h2>
            </a>
            <ul>
                <li><a href="/create_race"><button>{ "Создaть расу" }</button></a></li>
                <li><a href="/resource_calculator"><button>{ "Калькулятор ресурсов" }</button></a></li>
                <li><a href="/detachment_editor"><button>{ "Конструктор отрядов" }</button></a></li>
                <li><a href="/race_about_template"><button>{ "Шаблон \"Ваша страна как\"" }</button></a></li>
            </ul>
        </nav>
    }
}
