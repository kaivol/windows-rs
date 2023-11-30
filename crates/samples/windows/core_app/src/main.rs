#![windows_subsystem = "windows"]

use windows::{
    core::*,
    ApplicationModel::{Core::*, Package},
    Win32::{
        System::Com::*,
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
    },
    UI::Core::*,
};

#[implement(IFrameworkViewSource, IFrameworkView)]
struct CoreApp();

#[allow(non_snake_case)]
impl IFrameworkViewSource_Impl for CoreApp {
    fn CreateView(this: &Self::This) -> Result<IFrameworkView> {
        Ok(this.to_interface())
    }
}

#[allow(non_snake_case)]
impl IFrameworkView_Impl for CoreApp {
    fn Initialize(_this: &Self::This, _: Option<&CoreApplicationView>) -> Result<()> {
        Ok(())
    }

    fn Load(_this: &Self::This, _: &HSTRING) -> Result<()> {
        Ok(())
    }

    fn Uninitialize(_this: &Self::This) -> Result<()> {
        Ok(())
    }

    fn Run(_this: &Self::This) -> Result<()> {
        let window = CoreWindow::GetForCurrentThread()?;
        window.Activate()?;

        let dispatcher = window.Dispatcher()?;
        dispatcher.ProcessEvents(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }

    fn SetWindow(_this: &Self::This, _: Option<&CoreWindow>) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        if let Err(result) = Package::Current() {
            MessageBoxW(
                None,
                w!("This sample must be registered (via register.cmd) and launched from Start."),
                w!("Error"),
                MB_ICONSTOP | MB_OK,
            );
            return Err(result);
        }
    }

    let app: IFrameworkViewSource = CoreApp().into_interface();
    CoreApplication::Run(&app)?;
    Ok(())
}
