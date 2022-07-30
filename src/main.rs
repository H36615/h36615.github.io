use gloo::timers::callback::Timeout;
use rand::Rng;
use yew::prelude::*;

enum Message {
    Update,
}

struct Model {
    value: String,
    target_text: &'static str,
    single_update_in_millis: u32,
    updates_remaining: u32,
    timeout: Option<Timeout>,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().clone().send_message(Message::Update);

        let target_text = "H36615";
        Self {
            value: target_text.to_string(),
            target_text,
            single_update_in_millis: 30,
            updates_remaining: 14,
            timeout: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Message::Update => {
                self.updates_remaining = self.updates_remaining - 1;
                if self.updates_remaining > 0 {
                    self.value = self
                        .value
                        .chars()
                        .map(|_| rand::thread_rng().gen_range(b'A', b'Z') as char)
                        .collect();

                    self.timeout = Some({
                        let link = ctx.link().clone();
                        Timeout::new(self.single_update_in_millis, move || {
                            link.send_message(Message::Update)
                        })
                    });
                } else {
                    self.value = self.target_text.to_string();
                }

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ &self.value }</h1>
            </div>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
}

fn main() {
    yew::start_app::<Model>();
}
