use std::ptr;
use windows::core::*;
use windows::Win32::System::WinRT::*;

struct TestBuffer(std::cell::UnsafeCell<Vec<u8>>);

impl ComObjectImpl for TestBuffer {
    type Interfaces = (IMemoryBufferByteAccess,);
}

#[allow(non_snake_case)]
impl IMemoryBufferByteAccess_Impl for TestBuffer {
    fn GetBuffer(this: &Self::This, value: *mut *mut u8, capacity: *mut u32) -> Result<()> {
        unsafe {
            let vec = &mut *this.0.get();
            *value = vec.as_mut_ptr();
            *capacity = vec.len() as u32;
        }
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    let object = TestBuffer(vec![0xAA, 0xBB, 0xCC].into()).into_interface::<IMemoryBufferByteAccess>();

    let buffer = unsafe {
        let mut value = ptr::null_mut();
        let mut capacity = 0;
        object.GetBuffer(&mut value, &mut capacity)?;
        core::slice::from_raw_parts(value, capacity as usize)
    };

    assert_eq!(buffer, [0xAA, 0xBB, 0xCC]);
    Ok(())
}
