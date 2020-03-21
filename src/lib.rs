use seed::{*, prelude::*};

mod composer;
use composer::COMPOSERS;

#[wasm_bindgen(module = "/src/copy.js")]
extern "C" {
    #[wasm_bindgen(js_name = execCopyTarget)]
    fn exec_copy_target(target: web_sys::Node);

    #[wasm_bindgen(js_name = execCopyTarget)]
    fn exec_copy_event_target(target: web_sys::EventTarget);
}

struct Model {
    input_length: usize,
    input_text: Vec<char>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            input_length: 0,
            input_text: Vec::new(),
        }
    }
}

impl Model {
    fn copy_matched_composer(&self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let target = document.get_element_by_id("main-list").unwrap()
            .first_child().unwrap();
        exec_copy_target(target);
    }
}

#[derive(Clone)]
enum Msg {
    ChangeInputText(String),
    ExecCopy,
    NoOp,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeInputText(text) => {
            model.input_text = text.chars().collect::<Vec<char>>();
            model.input_length = model.input_text.len();
        },
        Msg::ExecCopy => model.copy_matched_composer(),
        Msg::NoOp => (),
    }
}


fn view(model: &Model) -> impl View<Msg> {
    vec![
        div![
            id!("command-region"),

            // テキストをコピーしたらメッセージを表示する
            div![
                id!("message"),
                "Press the [Enter] key or click a name!"
            ],

            div![
                input![
                    attrs!{
                        At::Id => "input-area",
                        At::Type => "text",
                        At::Placeholder => "Input a composer name",
                        At::AutoFocus => true.as_at_value(),
                    },
                    input_ev(Ev::Input, Msg::ChangeInputText),
                    keyboard_ev(Ev::KeyDown, |keyboard_event| {
                        if keyboard_event.key_code() == 13 { // 13 means the [ENTER] key.
                            Msg::ExecCopy
                        } else {
                            Msg::NoOp
                        }
                    }),
                ],

                button![
                    simple_ev(Ev::Click, Msg::ExecCopy),
                    "Copy!",
                ],
            ],
        ],

        // 作曲家リスト
        ul![
            id!("main-list"),
            {
                COMPOSERS.iter().filter(|text| {
                    text.chars().take(model.input_length)
                        .zip(model.input_text.iter())
                        .fold(true, |acc, (t, i)| acc && t.eq_ignore_ascii_case(i))
                }).map(|text| {
                    li![
                        text,
                        mouse_ev(Ev::Click, move |event| {
                            let target = event.target().unwrap();
                            exec_copy_event_target(target);
                            Msg::NoOp
                        }),
                    ]
                }).collect::<Vec<_>>()
            }
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}
