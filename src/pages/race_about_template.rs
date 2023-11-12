use std::f64::consts::PI;
use leptos::{*, wasm_bindgen::prelude::*};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, HtmlInputElement, Url, js_sys::Function};
use crate::GITHUB_ROOT;

fn draw_image_from_input(id: &str, x: f64, y: f64, w: f64, h: f64) {
    let name_img: HtmlInputElement = document()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| ())
        .unwrap();

    if let Some(files) = name_img.files() {
        let f = files.get(0).unwrap();
        let url = Url::create_object_url_with_blob(&f).unwrap();
        let img = HtmlImageElement::new().unwrap();
        img.set_src(&url);

        let on_load_text = format!("const ctx = document.getElementById('view').getContext('2d'); 
                                    const template = document.getElementById('template')
                                    ctx.drawImage(event.target,{},{},{},{});
                                    ctx.drawImage(template,{},{},{},{},{},{},{},{});
                                   ",x,y,w,h,x,y,w,h,x,y,w,h);
        let on_load = Function::new_with_args("event", &on_load_text);
        
        img.set_onload(Some(&on_load));
    }
}

#[component]
pub fn RaceAboutTemplate() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (color, set_color) = create_signal("".to_string());
    let (color_name, set_color_name) = create_signal("".to_string());
    let (flower, set_flower) = create_signal("".to_string());
    let (animal, set_animal) = create_signal("".to_string());
    let (px, set_px) = create_signal(9);
    let (py, set_py) = create_signal(9);
    let (view_url, set_view_url) = create_signal("".to_string());

    let create = move |_| {
        let document = web_sys::window().unwrap().document().unwrap();

        let template: HtmlImageElement = document
            .get_element_by_id("template")
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .map_err(|_| ())
            .unwrap();

        let canvas: HtmlCanvasElement = document
            .get_element_by_id("view")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        
        //color
        ctx.set_fill_style(&color.get().into());
        ctx.fill_rect(130.0, 740.0, 240.0, 280.0);
        //template
        let _ = ctx.draw_image_with_html_image_element(&template, 0.0, 0.0);
        ctx.set_font("40px Cera");
        ctx.set_text_align("center");
        //seting text color
        ctx.set_fill_style(&"#000".into());
        //name
        let _ = ctx.fill_text_with_max_width(&name.get(), 554.5, 622.0, 400.0);
        draw_image_from_input("name", 341.0, 145.0, 426.0, 434.0);
        //color
        ctx.set_font("30px Cera");
        let _ = ctx.fill_text_with_max_width(&color_name.get(), 250.0, 1060.0, 200.0);
        //flower
        let _ = ctx.fill_text_with_max_width(&flower.get(), 554.5, 1060.0, 200.0);
        draw_image_from_input("flower", 440.0, 750.0, 230.0, 270.0);
        //animal
        let _ = ctx.fill_text_with_max_width(&animal.get(), 859.0, 1060.0, 200.0);
        draw_image_from_input("animal", 743.0, 750.0, 230.0, 270.0);

        //polit coords
        ctx.set_fill_style(&"#fff".into());
        ctx.set_stroke_style(&"#fff".into());

        let size: f64 = 28.5;
        let x: f64 = 284.0+size*px.get() as f64;
        let y: f64 = 1201.0+size*py.get() as f64;

        ctx.begin_path();
        let _ = ctx.arc(x, y, size-10.0, 0.0, PI*2.0);
        ctx.stroke();
        ctx.begin_path();
        ctx.move_to(x, y);
        let _ = ctx.arc(x-0.5, y-0.5, size-13.0, 0.0, PI*2.0);
        ctx.fill();

        set_view_url.set(canvas.to_data_url().unwrap());
    };


    view! {
        <h1>"Шаблон \"Ваша страна как\""</h1>
        <div class="race-about-template-container">
            <table class="race-about-template-inputs">
                <tr>
                    <td>"Название страны"</td>
                    <td>
                        <input id="name" type="file" accept="image/*"/>
                        <input on:input=move |ev| { set_name.set(event_target_value(&ev)) } prop:value=name/>
                    </td>
                </tr>
                <tr>
                    <td>"Цвет"</td>
                    <td>
                        <input type="color"  on:input=move |ev| { set_color.set(event_target_value(&ev)) }/>
                        <input on:input=move |ev| { set_color_name.set(event_target_value(&ev)) } prop:value=color_name/>
                    </td>
                </tr>
                <tr>
                    <td>"Растение"</td>
                    <td>
                        <input id="flower" type="file" accept="image/*"/>
                        <input on:input=move |ev| { set_flower.set(event_target_value(&ev)) } prop:value=flower/>
                    </td>
                </tr>
                <tr>
                    <td>"Животное"</td>
                    <td>
                        <input id="animal" type="file" accept="image/*"/>
                        <input on:input=move |ev| { set_animal.set(event_target_value(&ev)) } prop:value=animal/>
                    </td>
                </tr>
                <tr>
                    <td>"Политические координаты"</td>
                    <td>
                        <label for="coords-vertical">"X"</label>
                        <input id="coords-vertical" type="range" min="0" max="18"
                        on:input=move |ev| { set_px.set(event_target_value(&ev).parse::<u8>().unwrap()) } prop:value=px/>
                        <label for="coords-horizontal">"Y"</label>
                        <input id="coords-horizontal" type="range" min="0" max="18"
                        on:input=move |ev| { set_py.set(event_target_value(&ev).parse::<u8>().unwrap()) } prop:value=py/>
                    </td>
                </tr>
                <tr>
                    <td colspan=2>
                        <button on:click=create>"Создать"</button>
                        <a href={move || view_url.get()} download="шаблон.png"><button on:click=create>"Скачать"</button></a>
                    </td>
                </tr>
            </table>
            <canvas id="view" width=1109 height=1844> </canvas>
        </div>
        <img src=format!("/{}/media/race_about_template.png", GITHUB_ROOT) id="template"/>
    }
}
