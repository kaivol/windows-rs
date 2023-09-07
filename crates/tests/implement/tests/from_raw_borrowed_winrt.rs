#![allow(non_snake_case)]

use std::sync::*;
use windows::{core::*, ApplicationModel::Background::*, Foundation::*, Win32::Foundation::*};

#[interface("a563f463-3d23-42cd-a2b5-6d21ee898aae")]
unsafe trait IBorrowed: IUnknown {
    unsafe fn Call(this: &Self::This) -> u32;
}

#[implement(IBackgroundTask, IBorrowed, IBackgroundTaskInstance)]
struct Borrowed(RwLock<u32>);

impl IBorrowed_Impl for Borrowed {
    unsafe fn Call(this: &Self::This) -> u32 {
        *this.0.read().unwrap()
    }
}

impl IBackgroundTask_Impl for Borrowed {
    fn Run(this: &Self::This, instance: Option<&IBackgroundTaskInstance>) -> Result<()> {
        if let Some(instance) = instance {
            assert_eq!(instance.SuspendedCount()?, *this.0.read().unwrap());
            Ok(())
        } else {
            Err(E_INVALIDARG.into())
        }
    }
}

impl IBackgroundTaskInstance_Impl for Borrowed {
    fn InstanceId(_this: &Self::This) -> Result<GUID> {
        unimplemented!()
    }
    fn Task(_this: &Self::This) -> Result<BackgroundTaskRegistration> {
        unimplemented!()
    }
    fn Progress(_this: &Self::This) -> Result<u32> {
        unimplemented!()
    }
    fn SetProgress(_this: &Self::This, _value: u32) -> Result<()> {
        unimplemented!()
    }
    fn TriggerDetails(_this: &Self::This) -> Result<IInspectable> {
        unimplemented!()
    }
    fn Canceled(
        _this: &Self::This,
        _cancelhandler: Option<&BackgroundTaskCanceledEventHandler>,
    ) -> Result<EventRegistrationToken> {
        unimplemented!()
    }
    fn RemoveCanceled(_this: &Self::This, _cookie: &EventRegistrationToken) -> Result<()> {
        unimplemented!()
    }
    fn SuspendedCount(this: &Self::This) -> Result<u32> {
        Ok(*this.0.read().unwrap())
    }
    fn GetDeferral(_this: &Self::This) -> Result<BackgroundTaskDeferral> {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let one_two_three: IBorrowed = Borrowed(RwLock::new(123)).into_interface();
        assert_eq!(one_two_three.Call(), 123);

        let task = one_two_three.cast::<IBackgroundTask>()?;
        let instance = one_two_three.cast::<IBackgroundTaskInstance>()?;

        assert_eq!(task.Run(None).unwrap_err().code(), E_INVALIDARG);
        task.Run(&instance)?;

        let handler = BackgroundTaskCanceledEventHandler::new(|instance, reason| {
            if let Some(instance) = instance {
                assert_eq!(
                    instance.SuspendedCount()?,
                    instance.cast::<IBorrowed>()?.Call()
                );
                assert_eq!(reason, BackgroundTaskCancellationReason::Abort);
            } else {
                assert_eq!(reason, BackgroundTaskCancellationReason::Terminating);
            }
            Ok(())
        });

        handler.Invoke(&instance, BackgroundTaskCancellationReason::Abort)?;
        handler.Invoke(None, BackgroundTaskCancellationReason::Terminating)?;

        Ok(())
    }
}
