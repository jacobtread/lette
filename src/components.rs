use sycamore::prelude::*;

pub struct AppContext {
    value: u32,
}

impl Default for AppContext {
    fn default() -> Self {
        Self { value: 123 }
    }
}

#[component]
pub fn Html<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        html {

        }
    }
}

#[derive(Prop)]
pub struct ButtonProps<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn Button<'a, G: Html>(cx: Scope<'a>, props: ButtonProps<'a, G>) -> View<G> {
    let app_context = use_context::<Signal<AppContext>>(cx);
    let children = props.children.call(cx);
    app_context.set(AppContext { value: 500 });
    view! { cx,
        button(href="https://example.com") {
            (children)
        }
    }
}

#[component]
pub fn Text<G: Html>(cx: Scope) -> View<G> {
    let app_context = use_context::<Signal<AppContext>>(cx);

    let value = app_context.get().value;

    view! { cx,
        p{
            (value)
        }
    }
}
