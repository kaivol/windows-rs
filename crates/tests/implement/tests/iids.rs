use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::marker::PhantomData;
use windows::core::*;
use windows::Foundation::Collections::{
    IIterable, IIterable_Impl, IIterator, IVectorView, IVectorView_Impl,
};
use windows::Win32::System::Com::IAgileObject;
use windows::Win32::System::WinRT::*;

#[implement(IBufferByteAccess, IIterable<T>, IVectorView<T>, ILanguageExceptionErrorInfo2)]
struct TestBuffer<T>(PhantomData<T>)
where
    T: ::windows::core::RuntimeType + 'static;

#[allow(non_snake_case)]
impl<T: ::windows::core::RuntimeType + 'static> IBufferByteAccess_Impl for TestBuffer<T> {
    fn Buffer(_this: &Self::This) -> Result<*mut u8> {
        unimplemented!()
    }
}

#[allow(non_snake_case)]
impl<T: ::windows::core::RuntimeType + 'static> IIterable_Impl<T> for TestBuffer<T> {
    fn First(_this: &Self::This) -> Result<IIterator<T>> {
        unimplemented!()
    }
}

#[allow(non_snake_case)]
impl<T: ::windows::core::RuntimeType + 'static> IVectorView_Impl<T> for TestBuffer<T> {
    fn GetAt(_this: &Self::This, _index: u32) -> Result<T> {
        unimplemented!()
    }

    fn Size(_this: &Self::This) -> Result<u32> {
        unimplemented!()
    }

    fn IndexOf(
        _this: &Self::This,
        _value: &<T as Type<T>>::Default,
        _index: &mut u32,
    ) -> Result<bool> {
        unimplemented!()
    }

    fn GetMany(
        _this: &Self::This,
        _start_index: u32,
        _items: &mut [<T as Type<T>>::Default],
    ) -> Result<u32> {
        unimplemented!()
    }
}

#[allow(non_snake_case)]
impl<T: ::windows::core::RuntimeType + 'static> ILanguageExceptionErrorInfo_Impl for TestBuffer<T> {
    fn GetLanguageException(_this: &Self::This) -> Result<IUnknown> {
        unimplemented!()
    }
}

#[allow(non_snake_case)]
impl<T: ::windows::core::RuntimeType + 'static> ILanguageExceptionErrorInfo2_Impl
    for TestBuffer<T>
{
    fn GetPreviousLanguageExceptionErrorInfo(
        _this: &Self::This,
    ) -> Result<ILanguageExceptionErrorInfo2> {
        unimplemented!()
    }

    fn CapturePropagationContext(
        _this: &Self::This,
        _languageexception: Option<&IUnknown>,
    ) -> Result<()> {
        unimplemented!()
    }

    fn GetPropagationContextHead(_this: &Self::This) -> Result<ILanguageExceptionErrorInfo2> {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<()> {
    let object = TestBuffer::<i32>(PhantomData::default()).into_object();
    let iinspectable = object.into_interface::<IInspectable>();

    let iids = iinspectable.GetIids()?;

    let mut iids = HashSet::<GUID, RandomState>::from_iter(iids.into_iter().copied());

    assert!(!iids.contains(&IIterable::<HSTRING>::IID));

    assert!(iids.remove(&IAgileObject::IID));
    assert!(iids.remove(&IWeakReferenceSource::IID));
    assert!(iids.remove(&IBufferByteAccess::IID));
    assert!(iids.remove(&IIterable::<i32>::IID));
    assert!(iids.remove(&IVectorView::<i32>::IID));
    assert!(iids.remove(&ILanguageExceptionErrorInfo::IID));
    assert!(iids.remove(&ILanguageExceptionErrorInfo2::IID));
    assert!(iids.is_empty());

    Ok(())
}
