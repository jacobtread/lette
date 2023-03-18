use sycamore::{prelude::*, render_to_string};

use crate::components::{AppContext, Button, Text};

mod components;

#[component]
fn MyExampleEmail<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Button {
            "Text"
        }
        Text {}

    }
}

#[test]
fn test() {
    let str = render_to_string(|cx| {
        let context = AppContext::default();
        let signal = create_signal(cx, context);
        provide_context_ref(cx, signal);
        MyExampleEmail(cx)
    });
    println!("{}", str)
}
