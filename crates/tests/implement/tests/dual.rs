#![allow(non_snake_case)]

use windows::{core::*, Foundation::*};

#[test]
fn implement() -> Result<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "hello".to_string(),
            sender,
        };

        let s: IStringable = t.into_interface();
        assert_eq!(s.ToString()?, "hello");

        let c: IClosable = s.cast()?;
        c.Close()?;
        assert!(receiver.recv().unwrap() == "close: hello");
    }
    assert!(receiver.recv().unwrap() == "drop: hello");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "world".to_string(),
            sender,
        };

        let c: IClosable = t.into_interface();
        c.Close()?;
        assert!(receiver.recv().unwrap() == "close: world");

        let s: IStringable = c.cast()?;
        assert!(s.ToString()? == "world");
    }
    assert!(receiver.recv().unwrap() == "drop: world");

    let (sender, receiver) = std::sync::mpsc::channel();
    {
        let t = Thing {
            value: "object".to_string(),
            sender,
        };

        let s: IStringable = t.into_interface();
        assert!(s.ToString()? == "object");

        // Confirms that the conversion to `IInspectable` properly handles
        // reference counting.
        let _: IInspectable = s.can_clone_into();
    }
    assert!(receiver.recv().unwrap() == "drop: object");

    Ok(())
}

#[implement(IStringable, IClosable)]
struct Thing {
    value: String,
    sender: std::sync::mpsc::Sender<String>,
}

impl Drop for Thing {
    fn drop(&mut self) {
        self.sender.send(format!("drop: {}", self.value)).unwrap();
    }
}

impl IStringable_Impl for Thing {
    fn ToString(this: &Self::This) -> Result<HSTRING> {
        Ok(HSTRING::from(&this.value))
    }
}

impl IClosable_Impl for Thing {
    fn Close(this: &Self::This) -> Result<()> {
        this.sender.send(format!("close: {}", this.value)).unwrap();
        Ok(())
    }
}
