use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::output::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates an output widget
#[derive(WidgetExt, InputExt, Debug)]
pub struct Output {
    _inner: *mut Fl_Output,
}

/// Creates a multiline-output widget
#[derive(WidgetExt, InputExt, Debug)]
pub struct MultilineOutput {
    _inner: *mut Fl_Multiline_Output,
}
