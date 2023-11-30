use windows::{
    core::*, Win32::Foundation::*, Win32::System::Com::*, Win32::System::Ole::*,
    Win32::System::SystemServices::*,
};

#[implement(IDataObject)]
struct DataObject();

impl IDataObject_Impl for DataObject {
    fn GetData(_this: &Self::This, _: *const FORMATETC) -> Result<STGMEDIUM> {
        unimplemented!()
    }
    fn GetDataHere(_this: &Self::This, _: *const FORMATETC, _: *mut STGMEDIUM) -> Result<()> {
        unimplemented!()
    }
    fn QueryGetData(_this: &Self::This, _: *const FORMATETC) -> HRESULT {
        unimplemented!()
    }
    fn GetCanonicalFormatEtc(
        _this: &Self::This,
        _: *const FORMATETC,
        _: *mut FORMATETC,
    ) -> HRESULT {
        unimplemented!()
    }
    fn SetData(
        _this: &Self::This,
        _: *const FORMATETC,
        _: *const STGMEDIUM,
        _: BOOL,
    ) -> Result<()> {
        unimplemented!()
    }
    fn EnumFormatEtc(_this: &Self::This, _: u32) -> Result<IEnumFORMATETC> {
        unimplemented!()
    }
    fn DAdvise(
        _this: &Self::This,
        format: *const FORMATETC,
        value: u32,
        sink: Option<&IAdviseSink>,
    ) -> Result<u32> {
        assert!(!format.is_null());
        assert_eq!(value, 789);
        assert!(sink.is_none());
        Ok(123)
    }
    fn DUnadvise(_this: &Self::This, _: u32) -> Result<()> {
        unimplemented!()
    }
    fn EnumDAdvise(_this: &Self::This) -> Result<IEnumSTATDATA> {
        unimplemented!()
    }
}

#[implement(IDropTarget)]
struct DropTarget();

impl IDropTarget_Impl for DropTarget {
    fn DragEnter(
        _this: &Self::This,
        object: Option<&IDataObject>,
        state: MODIFIERKEYS_FLAGS,
        point: &POINTL,
        effect: *mut DROPEFFECT,
    ) -> Result<()> {
        unsafe {
            assert_eq!(
                object.unwrap().DAdvise(&FORMATETC::default(), 789, None)?,
                123
            );
            assert_eq!(state, MK_MBUTTON);
            assert_eq!(*effect, DROPEFFECT_LINK);
            *effect = DROPEFFECT_MOVE;
            assert_eq!(*point, POINTL { x: 10, y: 20 });
            Ok(())
        }
    }
    fn DragOver(
        _this: &Self::This,
        _: MODIFIERKEYS_FLAGS,
        _: &POINTL,
        _: *mut DROPEFFECT,
    ) -> Result<()> {
        unimplemented!()
    }
    fn DragLeave(_this: &Self::This) -> Result<()> {
        Ok(())
    }
    fn Drop(
        _this: &Self::This,
        _: Option<&IDataObject>,
        _: MODIFIERKEYS_FLAGS,
        _: &POINTL,
        _: *mut DROPEFFECT,
    ) -> Result<()> {
        unimplemented!()
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let object: IDataObject = DataObject().into_interface();

        let target: IDropTarget = DropTarget().into_interface();
        target.DragLeave()?;

        let mut effect = DROPEFFECT_LINK;
        target.DragEnter(&object, MK_MBUTTON, POINTL { x: 10, y: 20 }, &mut effect)?;
        assert_eq!(effect, DROPEFFECT_MOVE);

        Ok(())
    }
}
