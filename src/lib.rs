use glium::{backend::Backend, SwapBuffersError};
use std::ffi::{c_void, CString};
use std::os::raw::c_char;

#[repr(C)]
pub struct CBackend<SB, GPA, GFD, IC, MC> {
    data: *mut c_void,
    swap_buffers: SB,
    get_proc_address: GPA,
    get_framebuffer_dimensions: GFD,
    is_current: IC,
    make_current: MC,
}

impl<SB, GPA, GFD, IC, MC> CBackend<SB, GPA, GFD, IC, MC>
where
    SB: Fn(*mut c_void) -> bool,
    GPA: Fn(*mut c_void, *const c_char) -> *const c_void,
    GFD: Fn(*mut c_void, &mut u32, &mut u32),
    IC: Fn(*mut c_void) -> bool,
    MC: Fn(*mut c_void),
{
    /// The lifecycle of this object must match the `OpenGLContext` lifetime and thus is unsafe to create.
    pub fn new(
        data: *mut c_void,
        swap_buffers: SB,
        get_proc_address: GPA,
        get_framebuffer_dimensions: GFD,
        is_current: IC,
        make_current: MC,
    ) -> Self {
        CBackend {
            data,
            swap_buffers,
            get_proc_address,
            get_framebuffer_dimensions,
            is_current,
            make_current,
        }
    }
}

unsafe impl<SB, GPA, GFD, IC, MC> Backend for CBackend<SB, GPA, GFD, IC, MC>
where
    SB: Fn(*mut c_void) -> bool,
    GPA: Fn(*mut c_void, *const c_char) -> *const c_void,
    GFD: Fn(*mut c_void, &mut u32, &mut u32),
    IC: Fn(*mut c_void) -> bool,
    MC: Fn(*mut c_void),
{
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        if !(self.swap_buffers)(self.data) {
            Err(SwapBuffersError::ContextLost)
        } else {
            Ok(())
        }
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        if let Ok(symbol) = CString::new(symbol) {
            (self.get_proc_address)(self.data, symbol.as_ptr())
        } else {
            std::ptr::null()
        }
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let mut width = u32::max_value();
        let mut height = u32::max_value();
        (self.get_framebuffer_dimensions)(self.data, &mut width, &mut height);
        if width == u32::max_value() || height == u32::max_value() {
            panic!("glium_c_backend::CBackend::get_framebuffer_dimensions(): C didnt set width and height");
        }
        (width, height)
    }

    fn is_current(&self) -> bool {
        (self.is_current)(self.data)
    }

    unsafe fn make_current(&self) {
        (self.make_current)(self.data)
    }
}
