use pebblerust::zero::*;
use pebblerust::types::*;
use pebblerust::c;

pub enum AppLogLevel {
  AppLogLevelError = 1,
  AppLogLevelWarning = 50,
  AppLogLevelInfo = 100,
  AppLogLevelDebug = 200,
  AppLogLevelDebugVerbose = 255,
}

pub fn app_event_loop() {
  unsafe {
    c::app_event_loop();
  }
}

pub fn window_create() -> *mut Window {
  unsafe {
    c::window_create()
  }
}

pub fn window_set_window_handlers(window: *mut Window, handlers: WindowHandlers) {
  unsafe {
    c::window_set_window_handlers(window, handlers);
  }
}

pub fn window_stack_push(window: *mut Window, animated: bool) {
  unsafe {
    c::window_stack_push(window, animated);
  }
}

pub fn app_log(level: AppLogLevel, msg: &str) {
  unsafe {
    let (filename, _): (*const u8, u32) = transmute("rusty");
    let (message, _): (*const u8, u32) = transmute(msg);
    c::app_log(level as u8, filename, 0, message);
  }
}

pub fn window_single_click_subscribe<T>(button: u8, subscriber: extern fn(ClickRecognizerRef, *mut T)) {
  unsafe {
    let subscriber_ptr: extern fn(ClickRecognizerRef, *mut u8) = transmute(subscriber);
    c::window_single_click_subscribe(button, subscriber_ptr);
  }
}

pub fn window_set_click_config_provider<T>(window: *mut Window, provider: extern fn(*mut T)) {
  unsafe {
    let provider_ptr: extern fn(*mut u8) = transmute(provider);
    c::window_set_click_config_provider(window, provider_ptr);
  }
}

pub fn window_get_root_layer(window: *mut Window) -> *mut Layer {
  unsafe {
    c::window_get_root_layer(window)
  }
}

pub fn window_set_background_color(window: *mut Window, color: GColor) {
    unsafe {
        c::window_set_background_color(window, color);
    }
}

pub fn layer_create(frame: GRect) -> *mut Layer {
    unsafe {
        c::layer_create(frame)
    }
}

pub fn layer_mark_dirty(layer: *mut Layer) {
    unsafe {
        c::layer_mark_dirty(layer);
    }
}

pub fn layer_set_update_proc(layer: *mut Layer, update_proc: LayerUpdateProc) {
    unsafe {
        c::layer_set_update_proc(layer, update_proc);
    }
}

pub fn layer_get_bounds(layer: *mut Layer) -> GRect {
  unsafe {
    c::layer_get_bounds(layer)
  }
}

pub fn text_layer_create(frame: GRect) -> *mut TextLayer {
  unsafe {
    c::text_layer_create(frame)
  }
}

pub fn text_layer_set_text(layer: *mut TextLayer, text: &str) {
  unsafe {
    c::text_layer_set_text(layer, text);
  }
}

pub fn text_layer_set_text_alignment(layer: *mut TextLayer, alignment: GTextAlignment) {
    unsafe {
        c::text_layer_set_text_alignment(layer, alignment);
    }
}

pub fn text_layer_set_background_color(layer: *mut TextLayer, color: GColor) {
    unsafe {
        c::text_layer_set_background_color(layer, color);
    }
}

pub fn text_layer_set_text_color(layer: *mut TextLayer, color: GColor) {
    unsafe {
        c::text_layer_set_text_color(layer, color);
    }
}

pub fn layer_add_child(parent: *mut Layer, child: *mut Layer) {
  unsafe {
    c::layer_add_child(parent, child);
  }
}

pub fn text_layer_get_layer(text_layer: *mut TextLayer) -> *mut Layer {
  unsafe {
    c::text_layer_get_layer(text_layer)
  }
}

pub fn window_set_click_config_provider_with_context<T>(window: *mut Window,
    provider: extern fn(*mut T), context: *mut T) {
  unsafe {
    let context_ptr: *mut u8 = transmute(context);
    let fn_ptr: extern fn(*mut u8) = transmute(provider);
    c::window_set_click_config_provider_with_context(window, fn_ptr, context_ptr);
  }
}

pub fn graphics_context_set_fill_color(context: *mut GContext, color: GColor) {
    unsafe {
        c::graphics_context_set_fill_color(context, color);
    }
}

pub fn graphics_draw_pixel(context: *mut GContext, point: GPoint) {
    unsafe {
        c::graphics_draw_pixel(context, point);
    }
}

fn corner_mask_to_bitfield(corner_mask: GCornerMask) -> u32 {
    // For some reason "|"'ng these values didn't work here..
    match corner_mask {
        GCornerMask::None => 0x00,
        GCornerMask::TopLeft => 0x01,
        GCornerMask::TopRight => 0x02,
        GCornerMask::BottomLeft => 0x04,
        GCornerMask::BottomRight => 0x08,
        GCornerMask::All => 0x0f,
        GCornerMask::Top => 0x03,
        GCornerMask::Bottom => 0x0c,
        GCornerMask::Left => 0x05,
        GCornerMask::Right => 0x0a
    }
}

pub fn graphics_fill_rect(context: *mut GContext, rect: GRect, corner_radius: u16, corner_mask: GCornerMask) {
    unsafe {
        c::graphics_fill_rect(context, rect, corner_radius, corner_mask_to_bitfield(corner_mask));
    }
}
