use std::ptr;
use windows::core::*;
use windows::Win32::Graphics::Direct3D10::*;

static mut HANDLER_INVOKED: bool = false;

struct Test;
impl ID3D10ShaderReflectionConstantBuffer_Impl for Test {
    fn GetDesc(&self, _pdesc: *mut D3D10_SHADER_BUFFER_DESC) -> Result<()> {
        unsafe { HANDLER_INVOKED = true };
        Ok(())
    }

    fn GetVariableByIndex(&self, _index: u32) -> Option<ID3D10ShaderReflectionVariable> {
        unimplemented!()
    }

    fn GetVariableByName(&self, _name: &PCSTR) -> Option<ID3D10ShaderReflectionVariable> {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<()> {
    let interface = ID3D10ShaderReflectionConstantBuffer::new(&Test);

    unsafe {
        interface.GetDesc(ptr::null_mut()).unwrap();
        assert!(HANDLER_INVOKED);
    }

    Ok(())
}
