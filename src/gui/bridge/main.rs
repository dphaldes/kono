#[cxx_qt::bridge]
mod ffi {

    unsafe extern "RustQt" {
        #[qobject]
        type Traa = super::TRAA;
    }
}

#[derive(Default)]
pub struct TRAA {}
