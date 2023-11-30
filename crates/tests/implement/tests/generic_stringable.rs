use windows::core::*;
use windows::Foundation::Collections::*;
use windows::Foundation::*;

#[implement(
    IIterable<IStringable>,
    IVectorView<IStringable>,
)]
struct Thing(Vec<IStringable>);

#[allow(non_snake_case)]
impl IVectorView_Impl<IStringable> for Thing {
    fn GetAt(this: &Self::This, index: u32) -> Result<IStringable> {
        this.0.get(index as usize).cloned().ok_or_else(|| panic!())
    }

    fn Size(_this: &Self::This) -> Result<u32> {
        panic!();
    }

    fn IndexOf(_this: &Self::This, _value: &Option<IStringable>, _index: &mut u32) -> Result<bool> {
        panic!();
    }

    fn GetMany(
        _this: &Self::This,
        _startindex: u32,
        _items: &mut [Option<IStringable>],
    ) -> Result<u32> {
        panic!();
    }
}

impl IIterable_Impl<IStringable> for Thing {
    fn First(_this: &Self::This) -> Result<IIterator<IStringable>> {
        unimplemented!()
    }
}

#[test]
fn test_implement() -> Result<()> {
    let url1: HSTRING = "http://one/".into();
    let url2: HSTRING = "http://two/".into();
    let url3: HSTRING = "http://three/".into();
    let v: IVectorView<IStringable> = Thing(vec![
        Uri::CreateUri(&url1)?.cast()?,
        Uri::CreateUri(&url2)?.cast()?,
        Uri::CreateUri(&url3)?.cast()?,
    ])
    .into_interface();

    assert_eq!("http://one/", v.GetAt(0)?.ToString()?);
    assert_eq!("http://two/", v.GetAt(1)?.ToString()?);
    assert_eq!("http://three/", v.GetAt(2)?.ToString()?);

    Ok(())
}
