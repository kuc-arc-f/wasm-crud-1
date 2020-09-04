mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-crud-1!");
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) ->i32{
    return x + y;
}

#[wasm_bindgen]
pub fn wasm_put_item(id_name: &str, title: &str, id_val: &str) -> Result<(), JsValue>{
    let document = web_sys::window().unwrap().document().unwrap();
    let entry_point = document.get_element_by_id(id_name).unwrap();
    let val = document.create_element("li")?;
    let s_title = format!("<h3 class='h3_title'>{}</h3>", title ); 
    let a_title = format!("<a href='/tasks/show/{}'>{}</a>", id_val, s_title ); 
    let s_id = format!("<span>ID :{}</span>", id_val ); 
    let btn_edit = format!("<a href='/tasks/edit/{}'> [ edit ] </a>", id_val ); 
    let s_elm = format!("<div class='div_post_wrap'>{}{}{}<hr /></div>", a_title, s_id ,btn_edit ); 
//console::log_1(&JsValue::from_str( &s_elm ));
    val.set_inner_html(&s_elm );
    entry_point.append_child(&val)?;

    Ok(())
}