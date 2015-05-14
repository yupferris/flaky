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
pub struct GContext;

pub type LayerUpdateProc = extern fn(*mut Layer, *mut GContext);

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

#[repr(C)]
pub enum GTextAlignment {
    Left,
    Center,
    Right
}

#[repr(C)]
pub enum GColor {
    Black = 0,
    White = 1,
    Clear = 0xffffffff
}

pub enum GCornerMask {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    All,
    Top,
    Bottom,
    Left,
    Right,
}

pub type ClickRecognizerRef = *mut ClickRecognizer;
