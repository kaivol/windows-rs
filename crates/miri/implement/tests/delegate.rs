use windows::core::*;
use windows::System::Threading::WorkItemHandler;

#[test]
fn test() -> Result<()> {
    static mut HANDLER_INVOKED: bool = false;
    let handler = WorkItemHandler::new(|_arg| {
        unsafe { HANDLER_INVOKED = true };
        Ok(())
    });

    handler.Invoke(None)?;

    unsafe {
        assert!(HANDLER_INVOKED);
    }

    Ok(())
}
