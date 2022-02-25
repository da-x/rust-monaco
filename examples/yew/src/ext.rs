//! Monaco editor as a [Yew](https://yew.rs) component.
//! Requires the "yew" feature.
use monaco::api::{CodeEditor as CodeEditorModel, TextModel};
use monaco::sys::editor::IStandaloneEditorConstructionOptions;
use std::rc::Rc;
use web_sys::HtmlElement;
use yew::{html, Callback, Component, Html, NodeRef, Properties, Context};

#[derive(Clone, Debug, Properties)]
pub struct CodeEditorProps {
    #[prop_or_default]
    pub options: Option<Rc<IStandaloneEditorConstructionOptions>>,

    #[prop_or_default]
    pub model: Option<TextModel>,
    /// This could be called multiple times if the `options` field changes.
    /// You can use this to initialise the editor
    #[prop_or_default]
    pub on_editor_created: Callback<Rc<CodeEditorModel>>,
}

impl PartialEq for CodeEditorProps {
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model
    }
}

/// CodeEditor component.
#[derive(Debug)]
pub struct CodeEditor {
    #[allow(unused)] // TODO
    props: CodeEditorProps,
    node_ref: NodeRef,
    editor: Option<Rc<CodeEditorModel>>,
}

impl Component for CodeEditor {
    type Message = ();
    type Properties = CodeEditorProps;

    fn create(ctx:&Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            node_ref: NodeRef::default(),
            editor: None,
        }
    }

    fn view(&self, _ctx:&Context<Self>) -> Html {
        let Self {
            node_ref, editor, ..
        } = self;

        debug_assert!(
            editor.is_none(),
            "previous editor must be disposed before re-creating"
        );

        html! {
            <div ref={node_ref.clone()} style="width: 100%; height: 100%;" />
        }
    }

    fn rendered(&mut self, ctx:&Context<Self>, _first_render: bool) {
        let el = self
            .node_ref
            .cast::<HtmlElement>()
            .expect("failed to resolve editor element");

        let props = ctx.props();
        let editor = CodeEditorModel::create_with_sys_options(&el, props.options.as_deref());

        if let Some(model) = &props.model {
            // initially we only update the model if it was actually given as a prop.
            // this way a value or model can be given in the options and it won't be
            // detached immediately
            editor.set_model(model)
        }

        let CodeEditorProps {
            on_editor_created,
            ..
        } = &ctx.props();

        let editor = Rc::new(editor);
        on_editor_created.emit(editor.clone());

        self.editor = Some(editor);
    }
}
