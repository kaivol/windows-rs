use windows::core::*;
use windows::Foundation::Collections::*;

#[implement(
    IKeyValuePair<i32, f32>,
)]
struct KeyValuePair();

#[allow(non_snake_case)]
impl IKeyValuePair_Impl<i32, f32> for KeyValuePair {
    fn Key(_this: &Self::This) -> Result<i32> {
        Ok(0)
    }
    fn Value(_this: &Self::This) -> Result<f32> {
        Ok(0.0)
    }
}

#[implement(
    IIterator<IKeyValuePair<i32, f32>>,
)]
struct Iterator();

#[allow(non_snake_case)]
impl IIterator_Impl<IKeyValuePair<i32, f32>> for Iterator {
    fn GetMany(_this: &Self::This, _items: &mut [Option<IKeyValuePair<i32, f32>>]) -> Result<u32> {
        Ok(0)
    }
    fn MoveNext(_this: &Self::This) -> Result<bool> {
        Ok(true)
    }
    fn Current(_this: &Self::This) -> Result<IKeyValuePair<i32, f32>> {
        Ok(KeyValuePair().into_interface())
    }
    fn HasCurrent(_this: &Self::This) -> Result<bool> {
        Ok(true)
    }
}

#[implement(
    IMapView<i32, f32>,
    IIterable<IKeyValuePair<i32, f32>>,
)]
struct MapView();

#[allow(non_snake_case)]
impl IMapView_Impl<i32, f32> for MapView {
    // TODO: shouldn't require & for primtiive
    fn HasKey(_this: &Self::This, _key: &i32) -> Result<bool> {
        Ok(true)
    }
    fn Lookup(_this: &Self::This, _key: &i32) -> Result<f32> {
        Ok(0.0)
    }
    fn Split(
        _this: &Self::This,
        _first: &mut Option<IMapView<i32, f32>>,
        _second: &mut Option<IMapView<i32, f32>>,
    ) -> Result<()> {
        Ok(())
    }
    fn Size(_this: &Self::This) -> Result<u32> {
        Ok(0)
    }
}

#[allow(non_snake_case)]
impl IIterable_Impl<IKeyValuePair<i32, f32>> for MapView {
    fn First(_this: &Self::This) -> Result<IIterator<IKeyValuePair<i32, f32>>> {
        Ok(Iterator().into_interface())
    }
}

#[implement(
    IMap<i32, f32>,
    IIterable<IKeyValuePair<i32, f32>>,
)]
struct Map();

#[allow(non_snake_case)]
impl IMap_Impl<i32, f32> for Map {
    fn Clear(_this: &Self::This) -> Result<()> {
        Ok(())
    }
    fn GetView(_this: &Self::This) -> Result<IMapView<i32, f32>> {
        Ok(MapView().into_interface())
    }
    fn HasKey(_this: &Self::This, _key: &i32) -> Result<bool> {
        Ok(true)
    }
    fn Insert(_this: &Self::This, _key: &i32, _value: &f32) -> Result<bool> {
        Ok(true)
    }
    fn Lookup(_this: &Self::This, _key: &i32) -> Result<f32> {
        Ok(0.0)
    }
    fn Remove(_this: &Self::This, _key: &i32) -> Result<()> {
        Ok(())
    }
    fn Size(_this: &Self::This) -> Result<u32> {
        Ok(0)
    }
}

impl IIterable_Impl<IKeyValuePair<i32, f32>> for Map {
    fn First(_this: &Self::This) -> Result<IIterator<IKeyValuePair<i32, f32>>> {
        Ok(Iterator().into_interface())
    }
}

fn main() -> Result<()> {
    let map: IMap<i32, f32> = Map().into_interface();
    map.Clear()?;
    map.HasKey(0)?;
    map.Insert(0, 0.0)?;
    map.Lookup(0)?;
    map.Remove(0)?;
    map.Size()?;
    map.First()?;

    let view = map.GetView()?;
    view.HasKey(0)?;
    view.Lookup(0)?;
    view.Split(&mut None, &mut None)?;
    view.Size()?;

    let iterator = view.First()?;
    iterator.GetMany(&mut [None])?;
    iterator.MoveNext()?;
    iterator.Current()?;
    iterator.HasCurrent()?;

    Ok(())
}
