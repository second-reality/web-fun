use rand::Rng;
use seed::{prelude::*, *};
use wasm_bindgen::Clamped;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(Msg::Rendered);
    orders.send_msg(Msg::AddCanvas);
    Model {
        render: 0,
        input: 100.,
        generate_noise: true,
        last_render_timestamp: 0.,
        all_canvas: vec![],
    }
}

struct Model {
    render: i32,
    input: f64,
    last_render_timestamp: f64,
    generate_noise: bool,
    all_canvas: Vec<ElRef<HtmlCanvasElement>>,
}

const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

enum Msg {
    Rendered(RenderInfo),
    RenderDelayChanged(String),
    GenerateNoiseToggle,
    AddCanvas,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered(info) => {
            if info.timestamp - model.last_render_timestamp > model.input {
                model.render += 1;
                model.last_render_timestamp = info.timestamp;

                for (id, canvas) in model.all_canvas.iter().enumerate() {
                    draw(canvas, model.generate_noise, id);
                }
            }
            orders.after_next_render(Msg::Rendered);
        }
        Msg::RenderDelayChanged(val) => {
            if let Ok(v) = val.parse() {
                if v > 0. {
                    model.input = v;
                }
            }
        }
        Msg::GenerateNoiseToggle => {
            model.generate_noise = !model.generate_noise;
        }
        Msg::AddCanvas => {
            model.all_canvas.push(ElRef::<HtmlCanvasElement>::default());
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, generate_noise: bool, canvas_id: usize) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    if generate_noise {
        let mut lol: Vec<u8> = vec![];
        // don't forget times 4 stupid!!!
        for _ in 0..WIDTH * HEIGHT {
            let mut rng = rand::thread_rng();
            //let color = 255 - (id * 10 % 255) as u8;
            let color = rng.gen::<u8>();
            lol.push(color);
            lol.push(color);
            lol.push(color);
            lol.push(200);
        }

        let data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&lol), WIDTH as u32, HEIGHT as u32);
        let data = data.unwrap();
        ctx.put_image_data(&data, 0., 0.).unwrap();
    } else {
        ctx.clear_rect(0., 0., WIDTH as f64, HEIGHT as f64);
    }

    ctx.set_fill_style(&JsValue::from_str("red"));
    ctx.set_font("30px Verdana");
    ctx.fill_text(
        &canvas_id.to_string(),
        WIDTH as f64 / 2.,
        HEIGHT as f64 / 2.,
    )
    .unwrap();
}

fn view(model: &Model) -> Node<Msg> {
    div![
        p!["This was rendered ", model.render, " times"],
        p![
            "last render timestamp (ms): ",
            model.last_render_timestamp as u64
        ],
        p!["Numer of canvas: ", model.all_canvas.len()],
        button!["add canvas", ev(Ev::Click, |_| Msg::AddCanvas)],
        input![
            attrs! {At::Type => "checkbox", At::Checked => model.generate_noise.as_at_value()},
            ev(Ev::Click, |_| Msg::GenerateNoiseToggle),
        ],
        "Generate Noise",
        div![
            "delay between updates ",
            input![
                attrs! {At::Type => "range", At::Min => 1, At::Max => 500, At::Value => model.input},
                input_ev(Ev::Input, Msg::RenderDelayChanged),
            ],
            "(",
            model.input,
            "ms",
            ")"
        ],
        hr!(),
        div![
            "TODO: read file using FileReader. Use Blob interface (duplicate content using a
            slice) to see if same file can be read several times."
        ],
        hr!(),
        model.all_canvas.iter().map(|c| view_one_canvas(c))
    ]
}

fn view_one_canvas(canvas: &ElRef<HtmlCanvasElement>) -> Node<Msg> {
    canvas![
        el_ref(canvas),
        attrs! {
            At::Width => px(WIDTH),
            At::Height => px(HEIGHT),
        },
        style![
            St::Border => "1px solid black",
        ],
    ]
}

fn main() {
    App::start("app", init, update, view);
}
