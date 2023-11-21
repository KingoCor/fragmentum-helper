use leptos::*;
use crate::aspects::{AspectClass, Aspect};
use crate::detachment::{DetachmentClass, Stats};

#[component]
pub fn StatsView<F>(
    stats: F
) -> impl IntoView 
where
    F: Fn() -> Stats + 'static,
{
    view! {
        {
            move || {
                let stats = stats();
                view!{
                    <td>{ stats.health }</td>
                    <td>{ stats.collision }</td>
                    <td>{ stats.scouting }</td>
                    <td>{ stats.maneuver }</td>
                    <td>{ stats.mobility }</td>
                    <td>{ stats.moral }</td>
                    <td>{ stats.protection_d }</td>
                    <td>{ stats.protection_m }</td>
                    <td>{ stats.damage_d }</td>
                    <td>{ stats.damage_m }</td>
                }
            }
        }
    }
}

#[component]
pub fn DetachmentClassView(
    stats: ReadSignal<Stats>,
    set_stats: WriteSignal<Stats>,
    set_name: WriteSignal<String>
) -> impl IntoView {

    let change_stats = move |value: &str| {
        let stats = match value {
            "Лёгкая пехоты" => Stats::get_by_detachment_class(DetachmentClass::LightInfantry),
            "Тяжёлая пехоты" => Stats::get_by_detachment_class(DetachmentClass::HeavyInfantry),
            "Лёгкая кавалерия" => Stats::get_by_detachment_class(DetachmentClass::LightCavalry),
            "Тяжёлая кавалерия" => Stats::get_by_detachment_class(DetachmentClass::HeavyCavalry),
            "Лёгкая поддержка" => Stats::get_by_detachment_class(DetachmentClass::LightSupport),
            _ => Stats::get_by_detachment_class(DetachmentClass::HeavySupport),
        };
        set_stats.set(stats);
        set_name.set(value.to_string());
    };

    view! {
        <tr>
            <td>
                <select on:change=move |ev| change_stats(&event_target_value(&ev))>
                    <option value="Лёгкая пехоты">"Лёгкая пехоты"</option>
                    <option value="Тяжёлая пехоты">"Тяжёлая пехоты"</option>
                    <option value="Лёгкая кавалерия">"Лёгкая кавалерия"</option>
                    <option value="Тяжёлая кавалерия">"Тяжёлая кавалерия"</option>
                    <option value="Лёгкая поддержка">"Лёгкая поддержка"</option>
                    <option value="Тяжёлая поддержка">"Тяжёлая поддержка"</option>
                </select>
            </td>
            <StatsView stats=move || stats.get()/> 
        </tr>
    }
}

#[component]
pub fn DetachmentLvlView(
    lvl: ReadSignal<i32>,
    set_lvl: WriteSignal<i32>
) -> impl IntoView {
    view! {
        <tr>
            <td>
                <select on:change=move |ev| set_lvl.set(event_target_value(&ev).parse::<i32>().unwrap())>
                    <option value="0">"Нет"</option>
                    <option value="1">"Регуляры"</option>
                    <option value="2">"Ветераны"</option>
                    <option value="3">"Элита"</option>
                </select>
            </td>
            <StatsView stats=move || Stats::one()*lvl.get()/> 
        </tr>
    }
}

#[component]
pub fn DetachmentAspectBonusView(
    class: AspectClass,
    is_negative: bool
) -> impl IntoView {
    let mut stats = Stats::get_aspect_bonus(class);
    if is_negative {
        stats = stats*-1;
    }
    let (stats, _) = create_signal(stats);

    view! {
        <tr>
            <td>{move || Aspect::get_class_default(class).name}</td>
            <StatsView stats=move || stats.get()/> 
        </tr>
    }
}

#[component]
pub fn DetachmentCustomStatsView(
    id: usize,
    name: String,
    stats: Stats,
    #[prop(into)]
    on_change: Callback<(usize,String,Stats)>,
    #[prop(into)]
    on_delete: Callback<usize>
) -> impl IntoView {
    let (stats, set_stats) = create_signal(stats);
    let (name, set_name) = create_signal(name);

    let sync_stats = move || {
        on_change.call((id,name.get(),stats.get()));
    };

    view! {
        <tr>
            <td>
                <a on:click=move |_| on_delete.call(id)>"✖"</a>
                <input value=move || name.get() on:change=move |ev| {
                    set_name.set(event_target_value(&ev));
                    sync_stats();
                }/>
            </td>
            <td><input type="number" value=move || stats.get().health on:change=move |ev| {
                let mut stats = stats.get();
                stats.health = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().collision on:change=move |ev| {
                let mut stats = stats.get();
                stats.collision = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().scouting on:change=move |ev| {
                let mut stats = stats.get();
                stats.scouting = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().maneuver on:change=move |ev| {
                let mut stats = stats.get();
                stats.maneuver = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().mobility on:change=move |ev| {
                let mut stats = stats.get();
                stats.mobility = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().moral on:change=move |ev| {
                let mut stats = stats.get();
                stats.moral = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().protection_d on:change=move |ev| {
                let mut stats = stats.get();
                stats.protection_d = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().protection_m on:change=move |ev| {
                let mut stats = stats.get();
                stats.protection_m = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().damage_d on:change=move |ev| {
                let mut stats = stats.get();
                stats.damage_d = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
            <td><input type="number" value=move || stats.get().damage_m on:change=move |ev| {
                let mut stats = stats.get();
                stats.damage_m = event_target_value(&ev).parse::<i32>().unwrap();
                set_stats.set(stats);
                sync_stats();
            }/></td>
        </tr>
    }
}
