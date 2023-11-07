use gloo_timers::callback::Timeout;
use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlDivElement};

// 消息类型枚举
pub enum ToastType {
    Success,
    Warning,
    Error,
}

fn get_svg_for_toast(toast_type: &ToastType) -> &'static str {
    match toast_type {
        ToastType::Success => {
            // SVG for success
            r#"<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>"#
        }
        ToastType::Warning => {
            // SVG for warning
            r#"<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>"#
        }
        ToastType::Error => {
            // SVG for error
            r#"<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current flex-shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>"#
        }
    }
}

// 显示消息的函数
pub fn show_toast(message: &str, toast_type: ToastType) {
    let toast_container = document().get_element_by_id("toast-container")
        .expect("toast-container must be present")
        .dyn_into::<HtmlElement>()
        .expect("toast-container should be an HTML element");

    let alert = document().create_element("div")
        .expect("Could not create div")
        .dyn_into::<HtmlElement>()
        .expect("Could not cast to HtmlElement");

    let alert_class = match toast_type {
        ToastType::Success => "alert-success",
        ToastType::Warning => "alert-warning",
        ToastType::Error => "alert-error",
    };

    let svg_html = get_svg_for_toast(&toast_type);

    alert.set_class_name(&format!("alert flex items-center p-4 {}", alert_class));
    alert.set_inner_html(&format!("{}<span>{}</span>", svg_html, message));
    
    toast_container.append_child(&alert).expect("Could not append alert to toast container");

    let alert_clone = alert.clone();
    // 设置定时器，3秒后移除 toast
    Timeout::new(3_000, move || {
        if let Err(e) = toast_container.remove_child(&alert_clone) {
            gloo_console::error!("Could not remove toast: {:?}", e);
        }
    }).forget();
}