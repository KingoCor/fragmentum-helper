use crate::aspects::Aspect;
use leptos::{*,ev::Event};

#[component]
fn PositiveButton(
    name: TextProp,
    cost: i32,
    #[prop(into)]
    on_click: Callback<Event>
) -> impl IntoView {
    view! {
        <li>
            <input type="radio"  
                id={name.get() + "_positive"} 
                name={name.get()}
                on:input=move |ev| on_click.call(ev)/>
            <label for={name.get() + "_positive"}>{cost}</label>
        </li>
    }
}

#[component]
fn NegativeButton(
    name: TextProp,
    cost: i32,
    #[prop(into)]
    on_click: Callback<Event>
) -> impl IntoView {
    view! {
        <li>
            <input type="radio"  
                id={name.get() + "_negative"} 
                name={name.get()}
                on:input=move |ev| on_click.call(ev)/>
            <label for={name.get() + "_negative"}>{-cost}</label>
        </li>
    }
}

#[component]
fn NoCostButton(
    name: TextProp,
    #[prop(into)]
    on_click: Callback<i32>
) -> impl IntoView {
    let (cost, set_cost) = create_signal(0);

    view! {
        <li>
            <input type="radio"  
                id={name.get() + "_no_cost"} 
                name={name.get()}
                on:click=move |_| on_click.call(cost.get())/>
            <input type="number" 
                on:input=move |ev| {set_cost.set(event_target_value(&ev).parse::<i32>().unwrap())}
                props:value=move || cost.get()/>
        </li>
    }
}


#[component]
pub fn AspectSelector(
    aspect: Aspect,
    #[prop(into)]
    choosed: Callback<Aspect>,
    #[prop(into)]
    removed: Callback<Aspect>
) -> impl IntoView  {
    let (aspect, _set_aspect) = create_signal(aspect.clone());

    let on_positive = move |_| {
        choosed.call(aspect.get_untracked());
    };
    let on_negative = move |_| {
        let mut aspect = aspect.get_untracked();
        aspect.cost *= -1;
        aspect.is_negative = true;
        choosed.call(aspect);
    };
    let on_no_cost = move |cost: i32| {
        let mut aspect = aspect.get_untracked();
        aspect.cost = cost;
        aspect.is_negative = true;
        choosed.call(aspect);
    };
    let on_not_selected = move |_| {
        removed.call(aspect.get_untracked());
    }; 

    view! {
        <fieldset class="aspect-selector"> 
            <legend>{ move || aspect.get_untracked().name }</legend>
            <div class="aspect-selector-description">
                <p>{ move || aspect.get_untracked().description }</p>
            </div>
            <ul class="aspect-selector-choose">
                <li>
                    <input type="radio" checked=true
                        id={ aspect.get_untracked().name + "_not_selected" } 
                        name={ aspect.get_untracked().name }
                        on:click=on_not_selected/>
                    <label for={ aspect.get_untracked().name + "_not_selected" }>{ "Не выбранно" }</label>
                </li>
                <Show when = move || { aspect.get_untracked().have_cost }>
                    <PositiveButton name=TextProp::from(aspect.get_untracked().name) cost=aspect.get_untracked().cost on_click=on_positive/>
                </Show>
                <Show when = move || { aspect.get_untracked().can_be_negative }>
                    <NegativeButton name=TextProp::from(aspect.get_untracked().name) cost=aspect.get_untracked().cost on_click=on_negative/>
                </Show>
                <Show when = move || { !aspect.get().have_cost }>
                    <NoCostButton name=TextProp::from(aspect.get_untracked().name) on_click=on_no_cost/>
                </Show>
            </ul>
        </fieldset>
    }
}
