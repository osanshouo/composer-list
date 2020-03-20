use seed::{*, prelude::*};

mod composer;
use composer::COMPOSERS;

#[wasm_bindgen(module = "/src/copy.js")]
extern "C" {
    #[wasm_bindgen(js_name = copyComposer)]
    fn copy_composer();

    #[wasm_bindgen(js_name = execCopy)]
    fn exec_copy(text: &str) -> bool;
}

const ENTER_KEY: u32 = 13;

struct Model {
    input_text: String,
    copy: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            input_text: "".to_string(),
            copy: false,
        }
    }
}

#[derive(Clone)]
enum Msg {
    ChangeInputText(String),
    PressEnterKey,
    PressOtherKey,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeInputText(text) => model.input_text = text.to_owned(),
        Msg::PressEnterKey => {
            model.copy = true;
            model.input_text = "".to_owned();
        },
        Msg::PressOtherKey => model.copy = false,
    }
}


fn view(model: &Model) -> impl View<Msg> {
    vec![
        // テキストをコピーしたらメッセージを表示する
        if model.copy { div![ "Copied!" ] } else { div!["Input a composer name, then press [Enter]!"] },

        input![
            attrs!{
                At::Type => "text",
                At::Placeholder => "Input a composer name, then press [Enter]",
                At::AutoFocus => true.as_at_value(),
            },
            input_ev(Ev::Input, Msg::ChangeInputText),
            keyboard_ev(Ev::KeyDown, |keyboard_event| {
                if keyboard_event.key_code() == ENTER_KEY {
                    copy_composer();
                    Msg::PressEnterKey
                } else {
                    Msg::PressOtherKey
                }
            }),
        ],

        // 作曲家リスト
        ul![
            id!("main-list"),
            {
                let input = model.input_text.chars().collect::<Vec<char>>();
                let n = input.len();

                COMPOSERS.iter().filter(|text| {
                    text.chars().take(n).zip(input.iter())
                        .fold(true, |acc, (t, i)| acc && t.eq_ignore_ascii_case(i))
                }).map(|text| {
                    li![
                        text,
                        mouse_ev(Ev::Click, move |_| {
                            exec_copy(*text);
                            Msg::PressEnterKey
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
