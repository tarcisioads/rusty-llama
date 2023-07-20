use leptos::{*, html::Input};

#[component]
pub fn TypeArea(cx:Scope, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView{
    let input_ref = create_node_ref::<Input>(cx);
    view!{cx,
        <div class="type-area">
           <form class="input-type-area" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");
           }
           >
                <input class="w-2/3 p-4 border border-gray-300 rounded-full" type="text" placeholder="Enter your prompt" node_ref=input_ref/>
                <input class="h-full p-4 bg-blue-500 text-white rounded-full cursor-pointer" type="submit"/>
           </form>
        </div>
    }
}
