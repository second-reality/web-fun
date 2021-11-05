use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    Model::default()
}

struct Model {
    counter: i32,
    canvas: ElRef<HtmlCanvasElement>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            counter: 0,
            canvas: ElRef::<HtmlCanvasElement>::default(),
        }
    }
}

enum Msg {
    Increment,
    Rendered,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => {
            model.counter += 1;
        }
        Msg::Rendered => {
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, counter: i32) {
    let colors = vec!["red", "blue", "green", "yellow"];
    let idx = (counter as usize) % colors.len();
    let c = colors[idx];
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    // clear canvas
    ctx.begin_path();
    ctx.clear_rect(0., 0., 400., 200.);

    let width = 200.;
    let height = 100.;

    ctx.rect(0., 0., width, height);
    ctx.set_fill_style(&JsValue::from_str(c));
    ctx.fill();

    ctx.move_to(0., 0.);
    ctx.line_to(width, height);
    ctx.stroke();
}

fn view(model: &Model) -> Node<Msg> {
    draw(&model.canvas, model.counter);
    div![
        C!["counter"],
        "This is a counter: ",
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
        canvas![
            el_ref(&model.canvas),
            attrs![
                At::Width => px(400),
                At::Height => px(200),
            ],
            style![
                St::Border => "1px solid black",
            ],
        ],
    ]
}

fn main() {
    App::start("app", init, update, view);
}
