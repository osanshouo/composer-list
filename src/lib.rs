use lazy_static::lazy_static;
use seed::{*, prelude::*};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

lazy_static!{
    static ref COMPOSERS: Vec<&'static str> = vec![
        "Beethoven, Ludwig van (1770.12.16-1827.3.27)",
        "Brahms, Johaness (1833.5.7-1897.4.3)",
        "Haydn, Franz Joseph (1732.3.31-1809.5.31)",
    ];
}

struct Model {
    prev_text: String,
    input_text: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            prev_text: "".to_string(),
            input_text: "".to_string(),
        }
    }
}

#[derive(Clone)]
enum Msg {
    ChangeInputText(String),
    KeyPress,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeInputText(text) => model.input_text = text.to_owned(),
        Msg::KeyPress => (),
    }
}


fn view(model: &Model) -> impl View<Msg> {
    div![
        input![ 
            attrs!{
                At::Type => "text",
                At::Placeholder => "Input a composer name, then press [Enter]",
                At::AutoFocus => true.as_at_value(),
            },
            input_ev(Ev::Input, Msg::ChangeInputText),
            // input_ev(Ev::KeyPress, |text| {
            //     model.prev_text = model.input_text;
            //     model.input_text = text;
            //     Msg::KeyPress
            // }),
        ],
        div![
            format!("Input Text: {}", model.input_text)
        ],
        ul![
            {
                let n = model.input_text.as_bytes().len();
                COMPOSERS.iter().filter(|text| {
                    text[0..n].eq_ignore_ascii_case(&model.input_text)
                }).map(|text| li![text]).collect::<Vec<_>>()
            }
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}
