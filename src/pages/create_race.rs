use std::collections::HashMap;
use std::cmp::Ordering;
use leptos::*;
use strum::IntoEnumIterator;
use crate::components::aspect_selector::AspectSelector;
use crate::aspects::{AspectClass,Aspect};

#[component]
pub fn CreateRace() -> impl IntoView {
    let (aspects, set_aspects) = create_signal(HashMap::<AspectClass,Aspect>::new());

    let get_cost = move || -> i32 {
        aspects.get().into_values().map(|v| -v.cost).sum::<i32>() + 2
    };

    view! {
        <h1>"Создание расы"</h1>
        <h2>"Выбор аспектов"</h2>
        <div class="aspect-selection">
            <div class="aspects-list">
            {
                AspectClass::iter()
                    .map(|class| view! {  
                        <AspectSelector 
                            aspect = Aspect::get_class_default(class) 
                            choosed = move |aspect: Aspect| { 
                                let mut aspects = aspects.get();
                                aspects.insert(aspect.class,aspect);
                                set_aspects.set(aspects);
                            }
                            removed = move |aspect: Aspect| { 
                                let mut aspects = aspects.get();
                                aspects.remove_entry(&aspect.class);
                                set_aspects.set(aspects);
                            }
                            />
                    }).collect::<Vec<_>>()
            }
            </div>
            <div class="selected-aspects">
                <h3 class= move || {
                    match aspects.get().len().cmp(&6) {
                        Ordering::Equal => "text-correct",
                        Ordering::Greater => "text-wrong",
                        Ordering::Less => ""
                    }
                }>
                    {move || format!("Выбрано ({}/6)",aspects.get().len())}
                </h3>
                <p class= move || {
                    match get_cost().cmp(&0) {
                        Ordering::Equal => "text-correct",
                        Ordering::Less => "text-wrong",
                        Ordering::Greater => ""
                    }
                }>
                    "Осталось очков: " { get_cost }
                </p>
                <ul>
                    {
                        move || aspects.get().into_values().map(|val| {view! {
                                <li>
                                    {format!("{} ({})",val.name,val.cost)}
                                </li>
                            }
                        }).collect::<Vec<_>>()
                    }
                </ul>
            </div>
        </div>
    }
}
