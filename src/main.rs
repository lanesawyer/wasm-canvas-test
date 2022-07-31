use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

const WIDTH: f64 = 400.0;
const HEIGHT: f64 = 400.0;

#[derive(Copy, Clone)]
struct Point<'a> {
    x: f64,
    y: f64,
    radius: f64,
    color: &'a str,
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().expect("document should have a body");

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut points = Vec::<Point>::new();
    for x in 0..1_000_000 {
        let random_x = js_sys::Math::random();
        let random_y = js_sys::Math::random();
        // draw_circle(&context, x as f64 * random_x, x as f64 * random_y, 4.0, "green");
        let point = Point {
            x: x as f64 * random_x,
            y: x as f64 * random_y,
            radius: 5.0,
            color: "green",
        };
        points.push(point);
    }

    let val = document.create_element("p").unwrap();
    val.set_text_content(Some("Generated data!"));
    body.append_child(&val).unwrap();

    for point in points {
        draw_circle(&context, point.x, point.y, point.radius, point.color);
    }

    let val = document.create_element("p").unwrap();
    val.set_text_content(Some("Drew data!"));
    body.append_child(&val).unwrap();
    
    context.translate(500.0, 500.0).unwrap();
}

// fn mouse_move() { {
//     let context = context.clone();
//     let pressed = pressed.clone();
//     let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
//         if pressed.get() {
//             context.line_to(event.offset_x() as f64, event.offset_y() as f64);
//             context.stroke();
//             context.begin_path();
//             context.move_to(event.offset_x() as f64, event.offset_y() as f64);
//         }
//     });
//     canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
//     closure.forget();
// }

fn draw_circle(context: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: &str) {
    // Skip drawing if it's not in the viewport
    if !(0.0..=WIDTH).contains(&x) { return }
    if !(0.0..=HEIGHT).contains(&y) { return }

    context.begin_path();

    context.arc(x, y, radius, 0.0, f64::consts::PI * 2.0).unwrap();
    context.set_fill_style(&JsValue::from_str(color));
    context.fill();

    context.set_line_width(5.0);
    context.set_stroke_style(&JsValue::from_str("#003300"));
    context.stroke();
}

fn main() {
    println!("Hello, world!");
}
