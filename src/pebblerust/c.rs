use pebblerust::types::*;

extern {
    pub fn app_event_loop();
    
    pub fn window_create() -> *mut Window;
    pub fn window_stack_push(window: *mut Window, animated: bool);
    pub fn app_log(level: u8, filename: *const u8, line_num: u32, msg: *const u8);
    pub fn window_set_click_config_provider(window: *mut Window, provider: extern fn(*mut u8));
    pub fn window_set_click_config_provider_with_context(window: *mut Window, provider: extern fn(*mut u8), context: *mut u8);
    pub fn window_single_click_subscribe(button: u8, subscriber: extern fn(ClickRecognizerRef, *mut u8));
    pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers);
    pub fn window_get_root_layer(window: *mut Window) -> *mut Layer;
    pub fn window_set_background_color(window: *mut Window, color: GColor);
    
    pub fn layer_get_bounds(layer: *mut Layer) -> GRect;
    pub fn layer_create(frame: GRect) -> *mut Layer;
    pub fn layer_mark_dirty(layer: *mut Layer);
    pub fn layer_set_update_proc(layer: *mut Layer, update_proc: LayerUpdateProc);
    
    pub fn text_layer_create(frame: GRect) -> *mut TextLayer;
    pub fn text_layer_set_text(layer: *mut TextLayer, text: &str);
    pub fn text_layer_set_text_alignment(layer: *mut TextLayer, alignment: GTextAlignment);
    pub fn text_layer_set_background_color(layer: *mut TextLayer, color: GColor);
    pub fn text_layer_set_text_color(layer: *mut TextLayer, color: GColor);
    pub fn layer_add_child(parent: *mut Layer, child: *mut Layer);
    pub fn text_layer_get_layer(text_layer: *mut TextLayer) -> *mut Layer;

    pub fn graphics_context_set_fill_color(context: *mut GContext, color: GColor);
    pub fn graphics_draw_pixel(context: *mut GContext, point: GPoint);
    pub fn graphics_fill_rect(context: *mut GContext, rect: GRect, corner_radius: u16, corner_mask: u32);
}


