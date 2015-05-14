#![crate_type="lib"]
#![feature(lang_items, intrinsics, no_std)]
#![no_std]

use pebblerust::lib::*;
use pebblerust::types::*;

#[lang="sized"]
trait Sized { }

#[lang="copy"]
trait Copy { }

mod pebblerust {
    pub mod lib;
    pub mod c;
    pub mod types;
    pub mod zero;
}

struct Position {
    x: i32,
    y: i32
}

struct Flake {
    pos: Position
}

extern fn window_load_handler(window: *mut Window) {
    let window_layer = window_get_root_layer(window);
    let window_bounds = layer_get_bounds(window_layer);

    let text_bounds = GRect {
        origin: GPoint { x: 0, y: 72 },
        size: GSize { w: window_bounds.size.w, h: 20 }
    };

    let flakes_layer = layer_create(window_bounds);

    layer_set_update_proc(flakes_layer, flakes_layer_update_proc);

    layer_add_child(window_layer, flakes_layer);
    
    let text_layer = text_layer_create(text_bounds);
    
    text_layer_set_text_alignment(text_layer, GTextAlignment::Center);
    text_layer_set_text(text_layer, "DAT DRIZZLE THO\0");
    text_layer_set_background_color(text_layer, GColor::Clear);
    text_layer_set_text_color(text_layer, GColor::White);
    
    layer_add_child(window_layer, text_layer_get_layer(text_layer));
}

extern fn window_unload_handler(_: *mut Window) { }
extern fn window_appear_handler(_: *mut Window) { }
extern fn window_disappear_handler(_: *mut Window) { }

static mut flake_pos: i32 = 0;

extern fn flakes_layer_update_proc(layer: *mut Layer, context: *mut GContext) {
    unsafe {
        graphics_context_set_fill_color(context, GColor::White);
        graphics_fill_rect(context, GRect { origin: GPoint { x: 40, y: flake_pos as u16 }, size: GSize { w: 20, h: 20 } }, 0, GCornerMask::None);
    
        //flake_pos += 1;
    
        layer_mark_dirty(layer); // yolo
    }
}

#[no_mangle]
pub extern fn main() -> i32 {
    let window = window_create();

    let window_handlers = WindowHandlers {
        load: window_load_handler,
        unload: window_unload_handler,
        appear: window_appear_handler,
        disappear: window_disappear_handler
    };
    window_set_window_handlers(window, window_handlers);

    window_set_background_color(window, GColor::Black);

    window_stack_push(window, true);

    app_event_loop();

    0
}
