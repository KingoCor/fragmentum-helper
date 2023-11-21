use std::collections::HashMap;
use leptos::*;
use crate::aspects::Aspect;
use crate::components::detachment_view::*;
use crate::components::io_state_settings::LoadState;
use crate::resources::Payment;
use crate::state::State;
use crate::detachment::{Stats,DetachmentClass};

#[component]
pub fn CreateDetachment() -> impl IntoView {
    let (class_name, set_class_name) = create_signal("Лёгкая пехоты".to_string());
    let (class_stats, set_class_stats) = create_signal(Stats::get_by_detachment_class(DetachmentClass::LightCavalry));
    let (lvl, set_lvl) = create_signal(0);
    let (aspects, set_aspects) = create_signal(Vec::<Aspect>::new());
    let (bonuses, set_bonuses) = create_signal(HashMap::<usize,(String,Stats)>::new());
    let (equipment, set_equipment) = create_signal(HashMap::<usize,(String,Stats)>::new());
    let (id, set_id) = create_signal(0);

    let load_aspects = move |state: State| {
        set_aspects.set(Vec::from_iter(state.aspects.into_values()));
    };

    let change_bonuses = move |(id, name, stats)| {
        let mut bonuses = bonuses.get_untracked();
        bonuses.insert(id, (name, stats));
        set_bonuses.set(bonuses);
    };

    let del_bonus = move |id| {
        let mut bonuses = bonuses.get_untracked();
        bonuses.remove(&id);
        set_bonuses.set(bonuses);
    };

    let change_equipment = move |(id, name, stats)| {
        let mut equipment = equipment.get_untracked();
        equipment.insert(id, (name, stats));
        set_equipment.set(equipment)
    };

    let del_equipmnet = move |id| {
        let mut equipment = equipment.get_untracked();
        equipment.remove(&id);
        set_equipment.set(equipment)
    };

    let stats_sum = move || {
        aspects.get().iter()
            .map(|aspect| {
                if aspect.is_negative {
                    Stats::get_aspect_bonus(aspect.class)*-1
                } else {
                    Stats::get_aspect_bonus(aspect.class)
                }
            }).sum::<Stats>()
        +bonuses.get().into_values().map(|(_,stats)| stats).sum()
        +equipment.get().into_values().map(|(_,stats)| stats).sum()
        +class_stats.get()
        +Stats::one()*lvl.get()
    };

    view! {
        <h1>"Конструктор отрядов"</h1>
        <LoadState on_load=load_aspects/>
        <table class="structure-selector">
            <thead>
                <tr>
                    <th>"Тип"</th>
                    <th>"Здоровье"</th>
                    <th>"Сшибка"</th>
                    <th>"Разведка"</th>
                    <th>"Манёвр"</th>
                    <th>"Мобильность"</th>
                    <th>"Мораль"</th>
                    <th>"Защита д/б"</th>
                    <th>"Защита б/б"</th>
                    <th>"урон д/б"</th>
                    <th>"урон б/б"</th>
                </tr>
            </thead>
            <tbody>
                <DetachmentClassView stats=class_stats set_stats=set_class_stats set_name=set_class_name/>
                <DetachmentLvlView lvl=lvl set_lvl=set_lvl/>
                <tr>
                    <td>"Аспекты"</td>
                    <td colspan=10></td>
                </tr>
                {
                    move || aspects
                        .get()
                        .iter()
                        .filter( |aspect| Stats::get_aspect_bonus(aspect.class)!=Stats::new() )
                        .map(|aspect| view! {
                            <DetachmentAspectBonusView class=aspect.class is_negative=aspect.is_negative/>
                        }).collect::<Vec<_>>()
                }
                {
                    move || bonuses
                        .get()
                        .iter()
                        .map(|(id,(name,stats))| view! {  
                            <DetachmentCustomStatsView id=*id name=name.to_string() stats=*stats on_change=change_bonuses on_delete=del_bonus/>
                        }).collect::<Vec<_>>()
                }
                <tr> 
                    <td colspan=11>
                        <a on:click=move |_| {
                            set_id.set(bonuses
                                       .get()
                                       .into_keys()
                                       .max()
                                       .unwrap_or(0)
                                       .max(equipment
                                            .get()
                                            .into_keys()
                                            .max()
                                            .unwrap_or(0)
                                            )+1);
                            let mut bonuses = bonuses.get();
                            bonuses.insert(id.get(), ("".to_string(),Stats::new()));
                            set_bonuses.set(bonuses);
                        }>"+ Добавить аспект"</a>
                    </td>
                </tr>
                <tr>
                    <td>"Снаряжение"</td>
                    <td colspan=10></td>
                </tr>
                {
                    move || equipment
                        .get()
                        .iter()
                        .map(|(id,(name,stats))| view! {  
                            <DetachmentCustomStatsView id=*id name=name.to_string() stats=*stats on_change=change_equipment on_delete=del_equipmnet/>
                        }).collect::<Vec<_>>()
                }
                <tr>
                    <td colspan=11>
                        <a on:click=move |_| {
                            set_id.set(bonuses
                                       .get()
                                       .into_keys()
                                       .max()
                                       .unwrap_or(0)
                                       .max(equipment
                                            .get()
                                            .into_keys()
                                            .max()
                                            .unwrap_or(0)
                                            )+1);
                            let mut equipment = equipment.get();
                            equipment.insert(id.get(), ("".to_string(),Stats::new()));
                            set_equipment.set(equipment);
                        }>"+ Добавить снаряжение"</a>
                    </td>
                </tr>
                <tr>
                    <td>"Итог"</td>
                    <StatsView stats=stats_sum/>
                </tr>
            </tbody> 
        </table>
        <div class="center-content">
            <textarea prop:value=move || {
                let mut aspect_stats = aspects
                    .get()
                    .iter()
                    .filter( |aspect| Stats::get_aspect_bonus(aspect.class)!=Stats::new() )
                    .map(|aspect| {
                        let mut text = aspect.name.clone();
                        let mut stats = Stats::get_aspect_bonus(aspect.class);
                        if aspect.is_negative { 
                            text = "(-)".to_string()+&text; 
                            stats = stats*-1;
                        }
                        text+&format!(" ({})", stats.to_string_short()).to_string()
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                if aspect_stats.len()!=0 { aspect_stats+="\n"; }

                let mut bonus_stats = bonuses
                    .get()
                    .into_values()
                    .map(|(name,stats)| {
                        format!("{} ({})", name.to_string(), stats.to_string_short())
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                if bonus_stats.len()!=0 { bonus_stats+="\n"; }

                let equipment_text = equipment
                    .get()
                    .into_values()
                    .map(|(name,stats)| {
                        let cost = stats.get_equipment_cost();
                        format!("{} ({}; Золото: {}, Метал: {}, Еда: {})", 
                                name.to_string(), 
                                stats.to_string_short(),
                                cost.money,
                                cost.metal,
                                cost.food)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let mut cost: Payment = equipment
                    .get()
                    .into_values()
                    .map(|(_,stats)| stats.get_equipment_cost())
                    .sum();

                cost.military_fragment = match lvl.get() {
                    0 => 1,
                    1 => 2,
                    3 => 4,
                    _ => 7
                };

                let have_cost = ((cost.money+cost.metal*10+cost.food*5)*(lvl.get()+1)*5)/100;

                format!(
                    "Тип: {}\n{}\n\n\
                    Обученость: {}\n\n\
                    Аспекты:\n{}{}\n\
                    Экипировка:\n{}\n\n\
                    Итоговые характеристики:\n{}\n\n\
                    Стоимость найма:\nЗолото: {}\nМетал: {}\nЕда: {}\nВФ: {}\n\n\
                    Стоимость содержания: {}",
                    class_name.get(),
                    class_stats.get().to_string(),
                    lvl.get(),
                    aspect_stats,
                    bonus_stats,
                    equipment_text,
                    stats_sum().to_string(),
                    cost.money,
                    cost.metal,
                    cost.food,
                    cost.military_fragment,
                    have_cost 
                )
            }/> 
        </div>
    }
}
