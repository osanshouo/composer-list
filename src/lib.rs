use seed::{*, prelude::*};

mod composer;
use composer::COMPOSERS;

#[wasm_bindgen(module = "/src/copy.js")]
extern "C" {
    #[wasm_bindgen(js_name = execCopy)]
    fn exec_copy(text: &str);
}

const ENTER_KEY: u32 = 13;

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
        match COMPOSERS.iter().find(|text| {
            text.chars().take(self.input_length)
                .zip(self.input_text.iter())
                .fold(true, |acc, (t, i)| acc && t.eq_ignore_ascii_case(i))
        }) {
            Some(text) => exec_copy(*text),
            None => (),
        };
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
        Msg::ExecCopy => {
            model.copy_matched_composer();
            model.input_text.clear();
            model.input_length = 0;
        },
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
                "Input a composer name, then press the [Enter] key!"
            ],

            div![
                input![
                    attrs!{
                        At::Id => "input-area",
                        At::Type => "text",
                        At::Placeholder => "Input a composer name, then press [Enter]",
                        At::AutoFocus => true.as_at_value(),
                    },
                    input_ev(Ev::Input, Msg::ChangeInputText),
                    keyboard_ev(Ev::KeyDown, |keyboard_event| {
                        if keyboard_event.key_code() == ENTER_KEY {
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
                        mouse_ev(Ev::Click, move |_| {
                            exec_copy(*text);
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
