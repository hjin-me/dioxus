use dioxus_core::prelude::*;

fn main() {}

trait SProps {}

trait Comp {
    type Props;
}

impl<T> Comp for FC<T> {
    type Props = T;
}

fn test() {
    // let g: FC<ButtonProps> = CustomButton;
}

trait Render: Sized {
    fn render(ctx: Context<Self>) -> VNode;
}
// include as much as you might accept
struct Button {
    onhover: Option<Box<dyn Fn()>>,
}

impl Render for Button {
    fn render(ctx: Context<Self>) -> VNode {
        let _onfocus = move |_evt: ()| log::debug!("Focused");

        // todo!()
        ctx.render(rsx! {
            button {
                // ..ctx.attrs,
                class: "abc123",
                // style: { a: 2, b: 3, c: 4 },
                onclick: move |_evt| {
                    // log::info("hello world");
                },
                // Custom1 { a: 123 }
                // Custom2 { a: 456, "abc", h1 {"1"}, h2 {"2"} }
                // Custom3 { a: "sometext goes here" }
                // Custom4 { onclick: |evt| log::info("click") }
            }
        })
    }
}

// #[fc]
// fn Button(ctx: Context, onhover: Option<&dyn Fn()>) -> VNode {}

// h1 {
//     tag: "type", abc: 123, class: "big small wide short",
//     "title1"
//     "title1"
//     "title1"
//     "title"
// }

//     h1 ("title") {
//          tag: "type",
//          abc: 123,
//          class: "big small wide short",
//     }

//     // <button
//     //     class="inline-block py-4 px-8 mr-6 leading-none text-white bg-indigo-600 hover:bg-indigo-900 font-semibold rounded shadow"
//     //     onclick={move |_| set_name("jill")}
//     //     onclick={move |_| set_name("jill")}
//     // >
//     //     "Jill!"
//     // </button>

//     button { "Jill!",
//         class: "inline-block py-4 px-8 mr-6 leading-none text-white bg-indigo-600 hover:bg-indigo-900 font-semibold rounded shadow"
//         onclick: move |_| set_name("jill"),
//         onclick: move |_| set_name("jill"),
//     }

//     button {
//         class: "inline-block py-4 px-8 mr-6 leading-none text-white bg-indigo-600 hover:bg-indigo-900 font-semibold rounded shadow"
//         onclick: move |_| set_name("jill"),
//         onclick: move |_| set_name("jill"),
//         // this is valid
//         "Jill!",
//         // this is also valid
//         {"Jill!"}
//     }

//     h1 { "Text", class: "inline-block py-4 px-8 mr-6 leading-none" }

//     // <h1 class="inline-block py-4 px-8 mr-6 leading-none">
//     //     "Text"
//     // </h1>

//     h1 {
//         div {
//             h1 {}
//             h2 {}
//             Brick {}

//             p {}
//             p {
//                 tag: "type",
//                 abc: 123,
//                 enabled: true,
//                 class: "big small wide short",

//                 a { "abcder" },
//                 h2 { "whatsup", class: "abc-123" },
//                 CustomComponent { a: 123, b: 456, key: "1" },
//             }

//             div { class: "big small wide short",
//                 div {},
//                 div {},
//                 div {},
//                 div {},
//             }
//         }
//     }

//     h2 {}
//     h3 {}
//     "abcd123"