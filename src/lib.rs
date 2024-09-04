use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn js_console_log(s: &str);
}

#[wasm_bindgen]
pub fn test(ws: WebSocket) -> Result<(), JsValue> {
	ws.send_with_u8_array(&[0,0,0,0,0,0])
}
