use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");
    let avatar_style = use_state(|| "profile1-neutral".to_string());
    
    let oninput = {
        let current_username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };
    
    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };
    
    let select_profile1 = {
        let avatar_style = avatar_style.clone();
        Callback::from(move |_| avatar_style.set("Avery".to_string()))
    };
    
    let select_profile2 = {
        let avatar_style = avatar_style.clone();
        Callback::from(move |_| avatar_style.set("Sadie".to_string()))
    };
    
    let select_profile3 = {
        let avatar_style = avatar_style.clone();
        Callback::from(move |_| avatar_style.set("George".to_string()))
    };
    
    let avatar_url = format!(
        "https://api.dicebear.com/9.x/big-smile/svg?seed={}",
        *avatar_style
    );

    html! {
       <div class="bg-gray-800 flex w-screen h-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div class="bg-white p-8 rounded-lg shadow-lg max-w-md w-full">
                    <h1 class="text-3xl font-bold text-center mb-6 text-violet-600">{"Welcome to ChatApp!"}</h1>
                    
                    <div class="flex justify-center mb-6">
                        <img src={avatar_url} class="w-32 h-32 rounded-full border-4 border-violet-200" alt="User Avatar" />
                    </div>
                    
                    <div class="flex justify-center space-x-4 mb-6">
                        <button onclick={select_profile1} class={classes!(
                            "p-2", "rounded-full", "transition-colors",
                            if *avatar_style == "profile1-neutral" { "bg-violet-200" } else { "bg-gray-100 hover:bg-gray-200" }
                        )}>
                            {"Profile 1"}
                        </button>
                        <button onclick={select_profile2} class={classes!(
                            "p-2", "rounded-full", "transition-colors",
                            if *avatar_style == "profile2" { "bg-violet-200" } else { "bg-gray-100 hover:bg-gray-200" }
                        )}>
                            {"Profile 2"}
                        </button>
                        <button onclick={select_profile3} class={classes!(
                            "p-2", "rounded-full", "transition-colors",
                            if *avatar_style == "big-smile" { "bg-violet-200" } else { "bg-gray-100 hover:bg-gray-200" }
                        )}>
                            {"Profile 3"}
                        </button>
                    </div>
                    
                    <div class="mb-6">
                        <input 
                            {oninput} 
                            class="w-full rounded-lg p-4 border-2 border-gray-300 focus:border-violet-500 focus:outline-none transition-colors" 
                            placeholder="Enter your username" 
                            value={(*username).clone()}
                        />
                    </div>
                    
                    <Link<Route> to={Route::Chat}> 
                        <button 
                            {onclick} 
                            disabled={username.len() < 1} 
                            class={classes!(
                                "w-full", "rounded-lg", "bg-violet-600", "text-white", "font-bold", "p-4", "uppercase",
                                "transition-colors", "hover:bg-violet-700",
                                if username.len() < 1 { "opacity-50 cursor-not-allowed" } else { "" }
                            )}
                        >
                            {"Start Chatting!"}
                        </button>
                    </Link<Route>>
                    
                    <p class="text-center text-gray-500 text-sm mt-4">
                        {"Connect with friends and colleagues in this simple chat app"}
                    </p>
                </div>
            </div>
        </div>
    }
}