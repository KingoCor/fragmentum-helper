use std::collections::HashMap;

use leptos::*;
use strum::IntoEnumIterator;
use crate::components::aspect_selector::AspectSelector;
use crate::aspects::{AspectClass,Aspect};

#[component]
pub fn CreateRace() -> impl IntoView {
    let (aspects, set_aspects) = create_signal(HashMap::<AspectClass,Aspect>::new());

    let get_cost = move || -> i32 {
        aspects.get().into_iter().map(|(_,v)| -1*v.cost).sum::<i32>() + 2
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
                    let count = aspects.get().len();
                    if count==6 {
                        "text-correct"
                    } else if count>6 {
                        "text-wrong"
                    } else {
                        ""
                    }
                }>
                    {move || format!("Выбрано ({}/6)",aspects.get().len())}
                </h3>
                <p class= move || {
                    let count = get_cost();
                    if count==0 {
                        "text-correct"
                    } else if count<0 {
                        "text-wrong"
                    } else {
                        ""
                    }
                }>
                    "Осталось очков: " { get_cost }
                </p>
                <ul>
                    {
                        move || aspects.get()
                            .into_iter()
                            .map(|(_,val)| {view! {
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
