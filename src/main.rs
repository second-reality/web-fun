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
            draw(&model.canvas);
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    // clear canvas
    ctx.begin_path();
    ctx.clear_rect(0., 0., 400., 200.);

    let width = 200.;
    let height = 100.;

    ctx.rect(0., 0., width, height);
    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill();

    ctx.move_to(0., 0.);
    ctx.line_to(width, height);
    ctx.stroke();
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

fn main() {
    App::start("app", init, update, view);
}
