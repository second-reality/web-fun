use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(Msg::Rendered);
    orders.send_msg(Msg::AddCanvas);
    Model {
        render: 0,
        input: 100.,
        last_render_timestamp: 0.,
        all_canvas: vec![],
    }
}

struct Model {
    render: i32,
    input: f64,
    last_render_timestamp: f64,
    all_canvas: Vec<ElRef<HtmlCanvasElement>>,
}

const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;

enum Msg {
    Rendered(RenderInfo),
    InputTextChanged(String),
    AddCanvas,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered(info) => {
            if info.timestamp - model.last_render_timestamp > model.input {
                model.render += 1;
                model.last_render_timestamp = info.timestamp;

                for canvas in model.all_canvas.iter() {
                    draw(canvas, model.render);
                }
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
        Msg::AddCanvas => {
            for _ in 0..10 {
                model.all_canvas.push(ElRef::<HtmlCanvasElement>::default());
            }
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, id: i32) {
    let colors: Vec<String> = (70..250)
        .step_by(10)
        .map(|i| format!("rgb({}, {}, {})", i, i, i))
        .collect();
    let idx = (id as usize) % colors.len();
    let c = &colors[idx];
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
        p![
            "last render timestamp (ms): ",
            model.last_render_timestamp as u64
        ],
        p!["Numer of canvas: ", model.all_canvas.len()],
        button!["add 10 canvas", ev(Ev::Click, |_| Msg::AddCanvas)],
        div![
            "delay between updates ",
            input![
                attrs!{At::Type => "range", At::Min => 1, At::Max => 500, At::Value => model.input},
                input_ev(Ev::Input, Msg::InputTextChanged),
            ],
            "(",
            model.input,
            "ms",
            ")"
        ],
        hr!(),
        div!["TODO: try using putImageData (see how faster it is, + use dedicated rust lib!)"],
        hr!(),
        model.all_canvas.iter().map(|c| one_canvas(c))
    ]
}

fn one_canvas(canvas: &ElRef<HtmlCanvasElement>) -> Node<Msg> {
    canvas![
        el_ref(canvas),
        attrs!{
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
