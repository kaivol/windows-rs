use windows::core::*;
use windows::Foundation::Collections::*;

// TODO: test whether we can implement two different IIterable's.

#[implement(
    IVectorView<i32>,
    IIterable<i32>,
)]
struct Thing();

#[allow(non_snake_case)]
impl IVectorView_Impl<i32> for Thing {
    fn GetAt(_this: &Self::This, index: u32) -> Result<i32> {
        Ok(index as i32)
    }

    fn Size(_this: &Self::This) -> Result<u32> {
        Ok(123)
    }

    fn IndexOf(_this: &Self::This, value: &i32, index: &mut u32) -> Result<bool> {
        *index = *value as u32;
        Ok(true)
    }

    fn GetMany(_this: &Self::This, _startindex: u32, _items: &mut [i32]) -> Result<u32> {
        panic!();
    }
}

impl IIterable_Impl<i32> for Thing {
    fn First(_this: &Self::This) -> Result<IIterator<i32>> {
        panic!();
    }
}

#[test]
fn test_implement() -> Result<()> {
    let v: IVectorView<i32> = Thing().into_interface();
    assert_eq!(012, v.GetAt(012)?);
    assert_eq!(123, v.Size()?);
    let mut index = 0;
    assert_eq!(true, v.IndexOf(456, &mut index)?);
    assert_eq!(456, index);

    Ok(())
}
