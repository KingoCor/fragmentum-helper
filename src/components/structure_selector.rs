use leptos::*;
use strum::IntoEnumIterator;
use std::collections::HashMap;
use crate::structures::{StructureClass, Structure};
use crate::resources::{Resource,Fragment};

#[component]
pub fn StructureView(
    structure: Structure,
    id: i32,
    #[prop(into)]
    on_click: Callback<i32>
) -> impl IntoView  {
    let (structure,set_structure) = create_signal(structure);
    let (id, _) = create_signal(id);

    view! {
        <tr>
            <td>{ structure.get().name}</td>
            <td>
                <input type="number"
                    value=move || structure.get().count
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.count = event_target_value(&ev).parse::<u32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <input type="number"
                    value=move || structure.get().revenue.population
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.revenue.population = event_target_value(&ev).parse::<i32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <input type="number"
                    value=move || structure.get().revenue.building_materials
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.cost.population = event_target_value(&ev).parse::<i32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <input type="number"
                    value=move || structure.get().revenue.money
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.cost.money = event_target_value(&ev).parse::<i32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <input type="number"
                    value=move || structure.get().revenue.food
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.cost.food = event_target_value(&ev).parse::<i32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <input type="number"
                    value=move || structure.get().revenue.metal
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.cost.metal = event_target_value(&ev).parse::<i32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <input type="number"
                    value=move || structure.get().revenue.military_fragment
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.cost.military_fragment = event_target_value(&ev).parse::<i32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <input type="number"
                    value=move || structure.get().revenue.political_fragment
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.cost.political_fragment = event_target_value(&ev).parse::<i32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <input type="number"
                    value=move || structure.get().revenue.economic_fragment
                    on:input=move |ev| {
                        let mut structure = structure.get();
                        structure.cost.economic_fragment = event_target_value(&ev).parse::<i32>().unwrap();
                        set_structure.set(structure);
                    }/>
            </td>
            <td>
                <a on:click=move |_| on_click.call(id.get())>"✖"</a>
            </td>
        </tr>
    }
}

#[component]
pub fn StructureSelector(
    structures: ReadSignal<HashMap::<i32,Structure>>,
    set_structures: WriteSignal<HashMap::<i32,Structure>>
) -> impl IntoView  {
    let (structure_id, set_structure_id) = create_signal(
        if let Some(id) = structures.get_untracked().into_keys().max() {
            id+1
        } else { 0 });

    let add_structure = move |structure: Structure| {
        set_structure_id.set(
            if let Some(id) = structures.get_untracked().into_keys().max() {
                id+1
            } else { 0 });
        let mut structures = structures.get();
        structures.insert(structure_id.get(), structure);
        set_structures.set(structures);
    };

    let delet_structure = move |id: i32| {
        let mut structures = structures.get();
        structures.remove(&id);
        set_structures.set(structures);
    };

    view! {
        <div class="add-structure">
        <h3>"Добавить: "</h3>
        {
            StructureClass::iter()
                    .map(|class| {
                        let structure = Structure::default(class);
                        let name = structure.name.clone();
                        view! {
                            <button on:click=move |_| add_structure(structure.clone())>
                                {name}
                            </button>
                        }
                    }).collect::<Vec<_>>()
        }
        </div>
        <table class="structure-selector">
            <thead>
                <tr>
                    <th rowspan=2>"Название"</th>
                    <th rowspan=2>"Кол-во"</th>
                    <th colspan=8>"Производство/Потребление"</th>
                    <th rowspan=2>"Удалить"</th>
                </tr>
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
            <tbody>
                {
                    move || structures.get().iter().map(|(key, val)| {view! {
                            <StructureView structure=val.clone() id=*key on_click=delet_structure />
                        }
                    }).collect::<Vec<_>>()
                }
            </tbody>
        </table>
    }
}
