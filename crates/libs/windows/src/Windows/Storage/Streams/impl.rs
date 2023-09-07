pub trait IBuffer_Impl: ::windows_core::BaseImpl {
    fn Capacity(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Length(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetLength(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Capacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Capacity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLength(this, value).into())
        }
        IBuffer_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Capacity: Capacity::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContentTypeProvider_Impl: ::windows_core::BaseImpl {
    fn ContentType(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IContentTypeProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContentTypeProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContentTypeProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContentTypeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContentTypeProvider_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ContentType: ContentType::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IDataReader_Impl: ::windows_core::BaseImpl {
    fn UnconsumedBufferLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn UnicodeEncoding(this: &Self::This) -> ::windows_core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(this: &Self::This, value: UnicodeEncoding) -> ::windows_core::Result<()>;
    fn ByteOrder(this: &Self::This) -> ::windows_core::Result<ByteOrder>;
    fn SetByteOrder(this: &Self::This, value: ByteOrder) -> ::windows_core::Result<()>;
    fn InputStreamOptions(this: &Self::This) -> ::windows_core::Result<InputStreamOptions>;
    fn SetInputStreamOptions(this: &Self::This, value: InputStreamOptions) -> ::windows_core::Result<()>;
    fn ReadByte(this: &Self::This) -> ::windows_core::Result<u8>;
    fn ReadBytes(this: &Self::This, value: &mut [u8]) -> ::windows_core::Result<()>;
    fn ReadBuffer(this: &Self::This, length: u32) -> ::windows_core::Result<IBuffer>;
    fn ReadBoolean(this: &Self::This) -> ::windows_core::Result<bool>;
    fn ReadGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn ReadInt16(this: &Self::This) -> ::windows_core::Result<i16>;
    fn ReadInt32(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ReadInt64(this: &Self::This) -> ::windows_core::Result<i64>;
    fn ReadUInt16(this: &Self::This) -> ::windows_core::Result<u16>;
    fn ReadUInt32(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ReadUInt64(this: &Self::This) -> ::windows_core::Result<u64>;
    fn ReadSingle(this: &Self::This) -> ::windows_core::Result<f32>;
    fn ReadDouble(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ReadString(this: &Self::This, codeunitcount: u32) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ReadDateTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::DateTime>;
    fn ReadTimeSpan(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::TimeSpan>;
    fn LoadAsync(this: &Self::This, count: u32) -> ::windows_core::Result<DataReaderLoadOperation>;
    fn DetachBuffer(this: &Self::This) -> ::windows_core::Result<IBuffer>;
    fn DetachStream(this: &Self::This) -> ::windows_core::Result<IInputStream>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IDataReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UnconsumedBufferLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnconsumedBufferLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnicodeEncoding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnicodeEncoding(this, value).into())
        }
        unsafe extern "system" fn ByteOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ByteOrder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetByteOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetByteOrder(this, value).into())
        }
        unsafe extern "system" fn InputStreamOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InputStreamOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InputStreamOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInputStreamOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InputStreamOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputStreamOptions(this, value).into())
        }
        unsafe extern "system" fn ReadByte<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadByte(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadBytes(this, ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&value), value_array_size as usize)).into())
        }
        unsafe extern "system" fn ReadBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadBuffer(this, length) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadBoolean<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadBoolean(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadInt16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadInt16(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadInt32(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadInt64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadInt64(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadUInt16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadUInt16(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadUInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadUInt32(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadUInt64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadUInt64(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadSingle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadSingle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadDouble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadDouble(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, codeunitcount: u32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadString(this, codeunitcount) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadDateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadDateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadTimeSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadTimeSpan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadAsync(this, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DetachBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DetachBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DetachStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DetachStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDataReader_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UnconsumedBufferLength: UnconsumedBufferLength::<Identity, Impl, OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Identity, Impl, OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Identity, Impl, OFFSET>,
            ByteOrder: ByteOrder::<Identity, Impl, OFFSET>,
            SetByteOrder: SetByteOrder::<Identity, Impl, OFFSET>,
            InputStreamOptions: InputStreamOptions::<Identity, Impl, OFFSET>,
            SetInputStreamOptions: SetInputStreamOptions::<Identity, Impl, OFFSET>,
            ReadByte: ReadByte::<Identity, Impl, OFFSET>,
            ReadBytes: ReadBytes::<Identity, Impl, OFFSET>,
            ReadBuffer: ReadBuffer::<Identity, Impl, OFFSET>,
            ReadBoolean: ReadBoolean::<Identity, Impl, OFFSET>,
            ReadGuid: ReadGuid::<Identity, Impl, OFFSET>,
            ReadInt16: ReadInt16::<Identity, Impl, OFFSET>,
            ReadInt32: ReadInt32::<Identity, Impl, OFFSET>,
            ReadInt64: ReadInt64::<Identity, Impl, OFFSET>,
            ReadUInt16: ReadUInt16::<Identity, Impl, OFFSET>,
            ReadUInt32: ReadUInt32::<Identity, Impl, OFFSET>,
            ReadUInt64: ReadUInt64::<Identity, Impl, OFFSET>,
            ReadSingle: ReadSingle::<Identity, Impl, OFFSET>,
            ReadDouble: ReadDouble::<Identity, Impl, OFFSET>,
            ReadString: ReadString::<Identity, Impl, OFFSET>,
            ReadDateTime: ReadDateTime::<Identity, Impl, OFFSET>,
            ReadTimeSpan: ReadTimeSpan::<Identity, Impl, OFFSET>,
            LoadAsync: LoadAsync::<Identity, Impl, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, Impl, OFFSET>,
            DetachStream: DetachStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IDataWriter_Impl: ::windows_core::BaseImpl {
    fn UnstoredBufferLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn UnicodeEncoding(this: &Self::This) -> ::windows_core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(this: &Self::This, value: UnicodeEncoding) -> ::windows_core::Result<()>;
    fn ByteOrder(this: &Self::This) -> ::windows_core::Result<ByteOrder>;
    fn SetByteOrder(this: &Self::This, value: ByteOrder) -> ::windows_core::Result<()>;
    fn WriteByte(this: &Self::This, value: u8) -> ::windows_core::Result<()>;
    fn WriteBytes(this: &Self::This, value: &[u8]) -> ::windows_core::Result<()>;
    fn WriteBuffer(this: &Self::This, buffer: ::core::option::Option<&IBuffer>) -> ::windows_core::Result<()>;
    fn WriteBufferRange(this: &Self::This, buffer: ::core::option::Option<&IBuffer>, start: u32, count: u32) -> ::windows_core::Result<()>;
    fn WriteBoolean(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn WriteGuid(this: &Self::This, value: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn WriteInt16(this: &Self::This, value: i16) -> ::windows_core::Result<()>;
    fn WriteInt32(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn WriteInt64(this: &Self::This, value: i64) -> ::windows_core::Result<()>;
    fn WriteUInt16(this: &Self::This, value: u16) -> ::windows_core::Result<()>;
    fn WriteUInt32(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn WriteUInt64(this: &Self::This, value: u64) -> ::windows_core::Result<()>;
    fn WriteSingle(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn WriteDouble(this: &Self::This, value: f64) -> ::windows_core::Result<()>;
    fn WriteDateTime(this: &Self::This, value: &super::super::Foundation::DateTime) -> ::windows_core::Result<()>;
    fn WriteTimeSpan(this: &Self::This, value: &super::super::Foundation::TimeSpan) -> ::windows_core::Result<()>;
    fn WriteString(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<u32>;
    fn MeasureString(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<u32>;
    fn StoreAsync(this: &Self::This) -> ::windows_core::Result<DataWriterStoreOperation>;
    fn FlushAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DetachBuffer(this: &Self::This) -> ::windows_core::Result<IBuffer>;
    fn DetachStream(this: &Self::This) -> ::windows_core::Result<IOutputStream>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IDataWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UnstoredBufferLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnstoredBufferLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnicodeEncoding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnicodeEncoding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: UnicodeEncoding) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnicodeEncoding(this, value).into())
        }
        unsafe extern "system" fn ByteOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ByteOrder) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ByteOrder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetByteOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ByteOrder) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetByteOrder(this, value).into())
        }
        unsafe extern "system" fn WriteByte<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteByte(this, value).into())
        }
        unsafe extern "system" fn WriteBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteBytes(this, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as usize)).into())
        }
        unsafe extern "system" fn WriteBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteBuffer(this, ::windows_core::from_raw_borrowed(&buffer)).into())
        }
        unsafe extern "system" fn WriteBufferRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, start: u32, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteBufferRange(this, ::windows_core::from_raw_borrowed(&buffer), start, count).into())
        }
        unsafe extern "system" fn WriteBoolean<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteBoolean(this, value).into())
        }
        unsafe extern "system" fn WriteGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteGuid(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn WriteInt16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteInt16(this, value).into())
        }
        unsafe extern "system" fn WriteInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteInt32(this, value).into())
        }
        unsafe extern "system" fn WriteInt64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteInt64(this, value).into())
        }
        unsafe extern "system" fn WriteUInt16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteUInt16(this, value).into())
        }
        unsafe extern "system" fn WriteUInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteUInt32(this, value).into())
        }
        unsafe extern "system" fn WriteUInt64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteUInt64(this, value).into())
        }
        unsafe extern "system" fn WriteSingle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteSingle(this, value).into())
        }
        unsafe extern "system" fn WriteDouble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteDouble(this, value).into())
        }
        unsafe extern "system" fn WriteDateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteDateTime(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn WriteTimeSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteTimeSpan(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn WriteString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteString(this, ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MeasureString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MeasureString(this, ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StoreAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StoreAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FlushAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FlushAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DetachBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DetachBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DetachStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DetachStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDataWriter_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UnstoredBufferLength: UnstoredBufferLength::<Identity, Impl, OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Identity, Impl, OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Identity, Impl, OFFSET>,
            ByteOrder: ByteOrder::<Identity, Impl, OFFSET>,
            SetByteOrder: SetByteOrder::<Identity, Impl, OFFSET>,
            WriteByte: WriteByte::<Identity, Impl, OFFSET>,
            WriteBytes: WriteBytes::<Identity, Impl, OFFSET>,
            WriteBuffer: WriteBuffer::<Identity, Impl, OFFSET>,
            WriteBufferRange: WriteBufferRange::<Identity, Impl, OFFSET>,
            WriteBoolean: WriteBoolean::<Identity, Impl, OFFSET>,
            WriteGuid: WriteGuid::<Identity, Impl, OFFSET>,
            WriteInt16: WriteInt16::<Identity, Impl, OFFSET>,
            WriteInt32: WriteInt32::<Identity, Impl, OFFSET>,
            WriteInt64: WriteInt64::<Identity, Impl, OFFSET>,
            WriteUInt16: WriteUInt16::<Identity, Impl, OFFSET>,
            WriteUInt32: WriteUInt32::<Identity, Impl, OFFSET>,
            WriteUInt64: WriteUInt64::<Identity, Impl, OFFSET>,
            WriteSingle: WriteSingle::<Identity, Impl, OFFSET>,
            WriteDouble: WriteDouble::<Identity, Impl, OFFSET>,
            WriteDateTime: WriteDateTime::<Identity, Impl, OFFSET>,
            WriteTimeSpan: WriteTimeSpan::<Identity, Impl, OFFSET>,
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            MeasureString: MeasureString::<Identity, Impl, OFFSET>,
            StoreAsync: StoreAsync::<Identity, Impl, OFFSET>,
            FlushAsync: FlushAsync::<Identity, Impl, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, Impl, OFFSET>,
            DetachStream: DetachStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IInputStream_Impl: ::windows_core::BaseImpl + super::super::Foundation::IClosable_Impl {
    fn ReadAsync(this: &Self::This, buffer: ::core::option::Option<&IBuffer>, count: u32, options: InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IInputStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInputStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInputStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInputStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadAsync(this, ::windows_core::from_raw_borrowed(&buffer), count, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInputStream_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ReadAsync: ReadAsync::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IInputStreamReference_Impl: ::windows_core::BaseImpl {
    fn OpenSequentialReadAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IInputStream>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IInputStreamReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInputStreamReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInputStreamReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenSequentialReadAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInputStreamReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenSequentialReadAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInputStreamReference_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenSequentialReadAsync: OpenSequentialReadAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IOutputStream_Impl: ::windows_core::BaseImpl + super::super::Foundation::IClosable_Impl {
    fn WriteAsync(this: &Self::This, buffer: ::core::option::Option<&IBuffer>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn FlushAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IOutputStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOutputStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WriteAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteAsync(this, ::windows_core::from_raw_borrowed(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FlushAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FlushAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOutputStream_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WriteAsync: WriteAsync::<Identity, Impl, OFFSET>,
            FlushAsync: FlushAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IPropertySetSerializer_Impl: ::windows_core::BaseImpl {
    fn Serialize(this: &Self::This, propertyset: ::core::option::Option<&super::super::Foundation::Collections::IPropertySet>) -> ::windows_core::Result<IBuffer>;
    fn Deserialize(this: &Self::This, propertyset: ::core::option::Option<&super::super::Foundation::Collections::IPropertySet>, buffer: ::core::option::Option<&IBuffer>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IPropertySetSerializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertySetSerializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Serialize(this, ::windows_core::from_raw_borrowed(&propertyset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Deserialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyset: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deserialize(this, ::windows_core::from_raw_borrowed(&propertyset), ::windows_core::from_raw_borrowed(&buffer)).into())
        }
        IPropertySetSerializer_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStream_Impl: ::windows_core::BaseImpl + super::super::Foundation::IClosable_Impl + IInputStream_Impl + IOutputStream_Impl {
    fn Size(this: &Self::This) -> ::windows_core::Result<u64>;
    fn SetSize(this: &Self::This, value: u64) -> ::windows_core::Result<()>;
    fn GetInputStreamAt(this: &Self::This, position: u64) -> ::windows_core::Result<IInputStream>;
    fn GetOutputStreamAt(this: &Self::This, position: u64) -> ::windows_core::Result<IOutputStream>;
    fn Position(this: &Self::This) -> ::windows_core::Result<u64>;
    fn Seek(this: &Self::This, position: u64) -> ::windows_core::Result<()>;
    fn CloneStream(this: &Self::This) -> ::windows_core::Result<IRandomAccessStream>;
    fn CanRead(this: &Self::This) -> ::windows_core::Result<bool>;
    fn CanWrite(this: &Self::This) -> ::windows_core::Result<bool>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IRandomAccessStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID, <IInputStream as ::windows_core::ComInterface>::IID, <IOutputStream as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRandomAccessStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, value).into())
        }
        unsafe extern "system" fn GetInputStreamAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInputStreamAt(this, position) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOutputStreamAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutputStreamAt(this, position) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Position<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Position(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, position).into())
        }
        unsafe extern "system" fn CloneStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CloneStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanRead(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanWrite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanWrite(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRandomAccessStream_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Size: Size::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            GetInputStreamAt: GetInputStreamAt::<Identity, Impl, OFFSET>,
            GetOutputStreamAt: GetOutputStreamAt::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            CloneStream: CloneStream::<Identity, Impl, OFFSET>,
            CanRead: CanRead::<Identity, Impl, OFFSET>,
            CanWrite: CanWrite::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamReference_Impl: ::windows_core::BaseImpl {
    fn OpenReadAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IRandomAccessStreamReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStreamReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRandomAccessStreamReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenReadAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStreamReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenReadAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRandomAccessStreamReference_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenReadAsync: OpenReadAsync::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamWithContentType_Impl: ::windows_core::BaseImpl + super::super::Foundation::IClosable_Impl + IContentTypeProvider_Impl + IInputStream_Impl + IOutputStream_Impl + IRandomAccessStream_Impl {}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IRandomAccessStreamWithContentType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<super::super::Foundation::IClosable as ::windows_core::ComInterface>::IID, <IContentTypeProvider as ::windows_core::ComInterface>::IID, <IInputStream as ::windows_core::ComInterface>::IID, <IOutputStream as ::windows_core::ComInterface>::IID, <IRandomAccessStream as ::windows_core::ComInterface>::IID];
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRandomAccessStreamWithContentType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRandomAccessStreamWithContentType {
    const VTABLE: Self::Vtable = { IRandomAccessStreamWithContentType_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
