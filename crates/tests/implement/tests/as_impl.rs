use windows::core::*;
use windows::Foundation::*;

#[implement(IStringable)]
struct Object1(u32);

impl IStringable_Impl for Object1 {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Object1".into())
    }
}

#[implement(IStringable)]
struct Object2(f32);

impl IStringable_Impl for Object2 {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Object2".into())
    }
}

#[test]
fn test() {
    let obj1: IStringable = Object1(1337).into();
    let obj2 = AsImpl::<Object2>::as_impl(&obj1);
    assert_eq!(obj2.0, f32::from_bits(1337));
}
