use js_sys::Function;
use monaco::{api::{CodeEditorOptions, CodeEditor as CodeEditorModel}, sys::{editor::{BuiltinTheme,
IStandaloneEditorConstructionOptions, IEditorOptionsWordWrap, LineNumbersType,
IEditorScrollbarOptions, IEditorScrollbarOptionsHorizontal, IEditorScrollbarOptionsVertical,
IEditorFindOptions, IEditorFindOptionsAutoFindInSelection, IEditorMinimapOptions,
IEditorOptionsCursorStyle, IEditorOptionsRenderLineHighlight,
IEditorOptionsAcceptSuggestionOnEnter, IPasteEvent, IEditorOptionsRenderWhitespace}, KeyCode, IPosition, Position}};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use yew::{html, Component, Html, Context};
use wasm_bindgen::closure::Closure;
mod ext;
use crate::ext::CodeEditor;

const CONTENT: &str = "A single line";

fn get_options() -> IStandaloneEditorConstructionOptions {
    let opts = CodeEditorOptions::default()
        .with_language("rust".to_owned())
        .with_value(CONTENT.to_owned())
        .with_builtin_theme(BuiltinTheme::VsDark)
        .to_sys_options();

    // https://github.com/vikyd/vue-monaco-singleline/blob/master/src/monaco-singleline.vue
    opts.set_word_wrap(Some(IEditorOptionsWordWrap::Off));
    opts.set_line_numbers(Some(LineNumbersType::Off));
    opts.set_line_decorations_width(Some(0.0));
    opts.set_overview_ruler_lanes(Some(0.0));
    opts.set_overview_ruler_border(Some(false));
    opts.set_scroll_beyond_last_column(Some(0.0));
    opts.set_line_numbers_min_chars(Some(0.0));
    opts.set_hide_cursor_in_overview_ruler(Some(true));
    opts.set_glyph_margin(Some(false));
    opts.set_folding(Some(false));
    opts.set_links(Some(false));
    opts.set_occurrences_highlight(Some(false));
    opts.set_cursor_style(Some(IEditorOptionsCursorStyle::LineThin));
    opts.set_render_line_highlight(Some(IEditorOptionsRenderLineHighlight::None));
    opts.set_render_whitespace(Some(IEditorOptionsRenderWhitespace::None));
    opts.set_render_indent_guides(Some(false));

    // opts.set_contextmenu(Some(false));
    opts.set_rounded_selection(Some(false));
    opts.set_accept_suggestion_on_enter(Some(IEditorOptionsAcceptSuggestionOnEnter::On));
    opts.set_automatic_layout(Some(true));
    opts.set_fixed_overflow_widgets(Some(true));
    // opts.set_word_based_suggestions(Some(false));

    let efo = IEditorFindOptions::default();
    efo.set_add_extra_space_on_top(Some(false));
    efo.set_auto_find_in_selection(Some(IEditorFindOptionsAutoFindInSelection::Never));
    efo.set_seed_search_string_from_selection(Some(false));
    opts.set_find(Some(&efo));

    let emo = IEditorMinimapOptions::default();
    emo.set_enabled(Some(false));
    opts.set_minimap(Some(&emo));

    let eso = IEditorScrollbarOptions::default();
    eso.set_horizontal(Some(IEditorScrollbarOptionsHorizontal::Hidden));
    eso.set_vertical(Some(IEditorScrollbarOptionsVertical::Hidden));
    eso.set_always_consume_mouse_wheel(Some(false));
    opts.set_scrollbar(Some(&eso));

    opts
}

struct App {
    options: Rc<IStandaloneEditorConstructionOptions>,
}

impl App {
    fn editor_created(editor: Rc<CodeEditorModel>) {
        Self::customize_for_single_line(editor);
    }

    fn customize_for_single_line(editor: Rc<CodeEditorModel>) {
        let enter = KeyCode::Enter.to_value() as f64;

        // https://github.com/vikyd/vue-monaco-singleline/blob/1de219c2f1ddd89f6b473e43716bbb3dfb662542/src/monaco-singleline.vue#L163

        let editor2 = editor.clone();
        let cb = Closure::wrap(Box::new(move || {
            let editor = (*editor2).as_ref();
            editor.trigger("", "acceptSelectedSuggestion", &JsValue::null());
        }) as Box<dyn FnMut()>);
        (*editor).as_ref().add_command(
            enter, cb.as_ref().unchecked_ref::<Function>(), None);
        cb.forget();

        let editor2 = editor.clone();
        let cb = Closure::wrap(Box::new(move |e: IPasteEvent| {
            let editor = (*editor2).as_ref();
            if e.range().end_line_number() <= 1.0 {
                return;
            }

            if let Some(model) = editor2.get_model() {
                let value = model.get_value();
                let value = value.replace("\n", " ");
                model.set_value(value.as_str());

                let pos = Position::new((value.len() + 1) as f64, 1 as f64);
                let pos2: JsValue = pos.into();
                editor.set_position(&JsCast::unchecked_into(pos2));
            }
        }) as Box<dyn FnMut(_)>);

        (*editor).as_ref().on_did_paste(cb.as_ref().unchecked_ref::<Function>());
        cb.forget();
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        Self { options: Rc::new(get_options()), }
    }

    fn changed(&mut self, _context: &Context<Self>) -> bool {
        false
    }

    fn view(&self, context: &Context<Self>) -> Html {
        log::info!("app viewed");

        let on_editor_created = context.link().callback(App::editor_created);

        html! {
            <div>
                { "text before" }
                 <div class="monaco-singleline">
                <CodeEditor options={ Rc::clone(&self.options) }
                    on_editor_created={ on_editor_created }/>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn start_app() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("Starting app");

    yew::start_app::<App>();
}
