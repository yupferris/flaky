pub type WindowHandler = extern fn(*mut Window);

#[repr(C)]
pub struct Window;
#[repr(C)]
pub struct Layer;
#[repr(C)]
pub struct TextLayer;
#[repr(C)]
pub struct ClickRecognizer;

#[repr(C)]
pub struct GPoint {
  pub x: u16,
  pub y: u16,
}

#[repr(C)]
pub struct GSize {
  pub w: u16,
  pub h: u16,
}

#[repr(C)]
pub struct GRect {
  pub origin: GPoint,
  pub size: GSize,
}

#[repr(C)]
pub struct WindowHandlers {
  pub load: WindowHandler,
  pub appear: WindowHandler,
  pub disappear: WindowHandler,
  pub unload: WindowHandler
}

pub type ClickRecognizerRef = *mut ClickRecognizer;
