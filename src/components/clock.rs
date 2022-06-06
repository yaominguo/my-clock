use super::flipper::Flippers;
use my_clock::{get_time, set_interval};
use sycamore::prelude::*;

#[component]
pub fn Clock<G: Html>(ctx: Scope) -> View<G> {
    // 初始化表盘及当前时间
    let flippers = Flippers::new();
    let time = create_signal(ctx, get_time("%H%M"));

    // 定时获取时间
    let handler: Box<dyn Fn() + 'static> = unsafe {
        std::mem::transmute(Box::new(move || time.set(get_time("%H%M"))) as Box<dyn Fn()>)
    };
    set_interval(handler);

    // 根据时间更新表盘并返回数据
    let list = create_memo(ctx, move || {
        flippers.check_time(&time.get());
        flippers
            .iter()
            .map(|flipper| flipper.clone())
            .collect::<Vec<_>>()
    });

    view! {
        ctx,
        div(class="clock") {
            Indexed {
                iterable: list,
                view: |ctx, flipper| view!{
                    ctx,
                    div(class={format!("flip {}", if *flipper.flipping.get() { "go" } else { "" })}) {
                        div(class={format!("digital front number{}", flipper.front.get())})
                        div(class={format!("digital back number{}", flipper.back.get())})
                    }
                },
            }
        }
    }
}
