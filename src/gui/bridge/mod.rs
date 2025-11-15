#[cxx_qt::bridge]
mod ffi {

    unsafe extern "RustQt" {
        #[qobject]
        type Dummy = super::DummyType;
    }
}

#[derive(Default)]
pub struct DummyType {}
