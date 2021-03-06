use rand::Rng;
use seed::{prelude::*, *};
use wasm_bindgen::Clamped;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;
use web_sys::{Blob, File};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(Msg::Rendered);
    orders.send_msg(Msg::AddCanvas);
    Model {
        render: 0,
        input: 50.,
        generate_noise: false,
        last_render_timestamp: 0.,
        all_canvas: vec![],
        input_file: None,
        file_content: String::new(),
        file_offset: 0,
    }
}

struct Model {
    render: i32,
    input: f64,
    last_render_timestamp: f64,
    generate_noise: bool,
    all_canvas: Vec<ElRef<HtmlCanvasElement>>,
    input_file: Option<File>,
    file_content: String,
    file_offset: i32,
}

const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

enum Msg {
    Rendered(RenderInfo),
    RenderDelayChanged(String),
    GenerateNoiseToggle,
    AddCanvas,
    FileUploaded(Option<File>),
    FileRead(String),
    ShowMoreFileContent,
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
        Msg::FileUploaded(Some(input_file)) => {
            model.input_file = Some(input_file);
            model.file_offset = 0;
            orders.after_next_render(|_| Msg::ShowMoreFileContent);
        }
        Msg::FileRead(text) => {
            model.file_content = text;
        }
        Msg::FileUploaded(None) => {
            log!("none file");
        }

        // Return response body as `Vec<u8>`.
        //
        // # Errors
        // Returns `FetchError::PromiseError`.
        //pub async fn bytes(&self) -> Result<Vec<u8>> {
        //    Ok(self
        //        .raw_response
        //        .array_buffer()
        //        .map_err(FetchError::PromiseError)
        //        .map(JsFuture::from)?
        //        .await
        //        .map_err(FetchError::PromiseError)
        //        .map(|array_buffer| js_sys::Uint8Array::new(&array_buffer))?
        //        .to_vec())
        //}
        Msg::ShowMoreFileContent => {
            if let Some(f) = &model.input_file {
                log!(f.name());
                let b: &Blob = f;
                log!(b.size());
                let slice = b
                    .slice_with_i32_and_i32(model.file_offset, model.file_offset + 120)
                    .unwrap();
                model.file_offset += 128;

                orders.perform_cmd(async move {
                    let text = JsFuture::from(slice.text())
                        .await
                        .expect("read file")
                        .as_string()
                        .expect("cast file text to String");
                    Msg::FileRead(text)
                });
            }
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, generate_noise: bool, canvas_id: usize) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    if generate_noise {
        let mut noise: Vec<u8> = vec![];
        for _ in 0..WIDTH * HEIGHT {
            let mut rng = rand::thread_rng();
            //let color = 255 - (id * 10 % 255) as u8;
            let color = rng.gen::<u8>();
            noise.push(color);
            noise.push(color);
            noise.push(color);
            noise.push(200);
        }

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&noise),
            WIDTH as u32,
            HEIGHT as u32,
        );
        let data = data.unwrap();
        ctx.put_image_data(&data, 0., 0.).unwrap();
    } else {
        ctx.clear_rect(0., 0., WIDTH as f64, HEIGHT as f64);
    }

    ctx.set_fill_style(&JsValue::from_str("red"));
    let font_size = 30;
    ctx.set_font(&(font_size.to_string() + "px Verdana"));
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
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
            button![
                "show more file content",
                ev(Ev::Click, |_| Msg::ShowMoreFileContent)
            ],
            input![
                attrs! {At::Type => "file"},
                ev(Ev::Change, |event| {
                    let file = event
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                        .and_then(|file_input| file_input.files())
                        .and_then(|file_list| file_list.get(0));

                    Msg::FileUploaded(file)
                }),
            ],
        ],
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
        div!["TODO: test async to fetch data from the web."],
        hr!(),
        model.all_canvas.iter().map(|c| view_one_canvas(c)),
        IF!(!model.file_content.is_empty() => view_file_content(&model.file_content))
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

fn view_file_content(content: &String) -> Node<Msg> {
    p!["file content is: ", content]
}

fn main() {
    App::start("app", init, update, view);
}
