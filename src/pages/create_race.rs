use std::collections::HashMap;
use std::cmp::Ordering;
use base64::Engine;
use leptos::*;
use strum::IntoEnumIterator;
use base64::engine::general_purpose::STANDARD;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::js_sys::Promise;
use web_sys::HtmlInputElement;
use crate::components::aspect_selector::AspectSelector;
use crate::components::structure_selector::StructureSelector;
use crate::aspects::{AspectClass,Aspect};
use crate::structures::Structure;
use crate::resources::{Resource,Fragment,Payment};
use crate::state::State;

#[component]
pub fn CreateRace() -> impl IntoView {
    let (aspects, set_aspects) = create_signal(HashMap::<AspectClass,Aspect>::new());
    let (structures, set_structures) = create_signal(<HashMap::<i32,Structure>>::new());
    let (resources, set_resources) = create_signal(Payment::default());

    let get_cost = move || -> i32 {
        aspects.get().into_values().map(|v| -v.cost).sum::<i32>() + 2
    };

    let load_file = create_action(move |input: &Promise| {
        let input = input.to_owned();
        async move {
            if let Ok(text) = JsFuture::from(input).await {
                if let Ok(state) = serde_json::from_str::<State>(&text.as_string().unwrap()) {
                    set_aspects.set(state.aspects);
                    set_resources.set(state.resources);
                    let mut structures = HashMap::<i32,Structure>::new();
                    for (i,s) in state.structures.iter().enumerate() {
                        structures.insert(i as i32, s.clone());
                    }
                    set_structures.set(structures);
                }
            }

        }
    });

    view! {
        <h1>"Создание государства"</h1>
        <div class="center-content">
            <p>"Загрузить файл настроек: "</p>
            <input type="file" accept=".json" on:input=move |ev| {
                let input = event_target::<HtmlInputElement>(&ev);
                if let Some(files) = input.files() {
                    let file = files.get(0).unwrap();
                    load_file.dispatch(file.text());
                }
            }/>
        </div>
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
        <h2>"Добавление построек"</h2>
        <StructureSelector structures=structures set_structures=set_structures/>
        <h2>"Ресурсы"</h2>
        <table class="resource-editor">
            <thead>
                <tr>
                    <th>"Население"</th>
                    <th>"Стройматериалы"</th>
                    <th>"Золото"</th>
                    <th>"Еда"</th>
                    <th>"Метал"</th>
                    <th>"ВФ"</th>
                    <th>"ПФ"</th>
                    <th>"ЭФ"</th>
                </tr>
            </thead>
            <tr>
                <td><input type="number"
                    value=move || { 
                        let r = resources.get();
                        if let Resource::Population(n) = r.population {
                            n
                        } else { 0 }
                    }
                    on:input=move |ev| {
                        let mut r = resources.get();
                        r.population = Resource::Population(event_target_value(&ev).parse::<i32>().unwrap());
                        set_resources.set(r);
                    }
                /></td>
                <td><input type="number"
                    value=move || { 
                        let r = resources.get();
                        if let Resource::BuildingMaterials(n) = r.building_materials {
                            n
                        } else { 0 }
                    }
                    on:input=move |ev| {
                        let mut r = resources.get();
                        r.building_materials = Resource::BuildingMaterials(event_target_value(&ev).parse::<i32>().unwrap());
                        set_resources.set(r);
                    }
                /></td>
                <td><input type="number"
                    value=move || { 
                        let r = resources.get();
                        if let Resource::Money(n) = r.money {
                            n
                        } else { 0 }
                    }
                    on:input=move |ev| {
                        let mut r = resources.get();
                        r.money = Resource::Money(event_target_value(&ev).parse::<i32>().unwrap());
                        set_resources.set(r);
                    }
                /></td>
                <td><input type="number"
                    value=move || { 
                        let r = resources.get();
                        if let Resource::Food(n) = r.food {
                            n
                        } else { 0 }
                    }
                    on:input=move |ev| {
                        let mut r = resources.get();
                        r.food = Resource::Food(event_target_value(&ev).parse::<i32>().unwrap());
                        set_resources.set(r);
                    }
                /></td>
                <td><input type="number"
                    value=move || { 
                        let r = resources.get();
                        if let Resource::Metal(n) = r.metal {
                            n
                        } else { 0 }
                    }
                    on:input=move |ev| {
                        let mut r = resources.get();
                        r.metal = Resource::Metal(event_target_value(&ev).parse::<i32>().unwrap());
                        set_resources.set(r);
                    }
                /></td>
                <td><input type="number"
                    value=move || { 
                        let r = resources.get();
                        if let Fragment::Military(n) = r.military_fragment {
                            n
                        } else { 0 }
                    }
                    on:input=move |ev| {
                        let mut r = resources.get();
                        r.military_fragment = Fragment::Military(event_target_value(&ev).parse::<i32>().unwrap());
                        set_resources.set(r);
                    }
                /></td>
                <td><input type="number"
                    value=move || { 
                        let r = resources.get();
                        if let Fragment::Political(n) = r.political_fragment {
                            n
                        } else { 0 }
                    }
                    on:input=move |ev| {
                        let mut r = resources.get();
                        r.political_fragment = Fragment::Political(event_target_value(&ev).parse::<i32>().unwrap());
                        set_resources.set(r);
                    }
                /></td>
                <td><input type="number"
                    value=move || { 
                        let r = resources.get();
                        if let Fragment::Economic(n) = r.economic_fragment {
                            n
                        } else { 0 }
                    }
                    on:input=move |ev| {
                        let mut r = resources.get();
                        r.economic_fragment = Fragment::Economic(event_target_value(&ev).parse::<i32>().unwrap());
                        set_resources.set(r);
                    }
                /></td>
            </tr>
        </table>
        <div class="center-content">
            <a download="Страна.json" href=move || {
                let data_json = serde_json::to_string(&State{
                    aspects: aspects.get(),
                    resources: resources.get(),
                    structures: Vec::<Structure>::from_iter(structures.get().into_values())
                }).unwrap();
                let mut data_base64 = String::new();
                STANDARD.encode_string(data_json, &mut data_base64);
                let file = "data:application/octet-stream;charset=utf-16le;base64,".to_string() + &data_base64;

                file
            }><button>"Скачать файл с настройками страны"</button></a>
        </div>
    }
}
