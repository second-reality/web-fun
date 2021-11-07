use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(Msg::Rendered);
    let mut model = Model::default();
    model.input = 500.;
    model
}

const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;

#[derive(Default)]
struct Model {
    render: i32,
    input: f64,
    last_render_timestamp: f64,
    canvas: ElRef<HtmlCanvasElement>,
}

enum Msg {
    Rendered(RenderInfo),
    InputTextChanged(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered(info) => {
            if info.timestamp - model.last_render_timestamp > model.input {
                model.render += 1;
                model.last_render_timestamp = info.timestamp;
                draw(&model.canvas, model.render);
            }
            orders.after_next_render(Msg::Rendered);
        }
        Msg::InputTextChanged(val) => {
            if let Ok(v) = val.parse() {
                if v > 0. {
                    model.input = v;
                }
            }
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, id: i32) {
    let colors = vec!["red", "blue", "green", "yellow"];
    let idx = (id as usize) % colors.len();
    let c = colors[idx];
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    let width = WIDTH as f64;
    let height = HEIGHT as f64;

    // clear canvas
    ctx.begin_path();
    ctx.clear_rect(0., 0., width, height);

    ctx.rect(0., 0., width, height);
    ctx.set_fill_style(&JsValue::from_str(c));
    ctx.fill();

    ctx.move_to(0., 0.);
    ctx.line_to(width, height);
    ctx.stroke();
}

fn view(model: &Model) -> Node<Msg> {
    div![
        p!["This was rendered ", model.render, " times"],
        p!["last render timestamp: ", model.last_render_timestamp],
        div![
            "delay between updates",
            input![
                attrs![At::Type => "Number", At::Value => model.input, At::Step => 10],
                input_ev(Ev::Input, Msg::InputTextChanged),
            ],
        ],
        one_canvas(&model.canvas),
    ]
}

fn one_canvas(canvas: &ElRef<HtmlCanvasElement>) -> Node<Msg> {
    div![canvas![
        el_ref(&canvas),
        attrs![
            At::Width => px(WIDTH),
            At::Height => px(HEIGHT),
        ],
        style![
            St::Border => "1px solid black",
        ],
    ],]
}

fn main() {
    App::start("app", init, update, view);
}
