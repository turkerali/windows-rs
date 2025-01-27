#[cfg(feature = "Devices_Adc_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdcChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdcChannel {
    type Vtable = IAdcChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdcChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x040bf414_2588_4a56_abef_73a260acc60a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcChannel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReadValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ReadRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdcController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdcController {
    type Vtable = IAdcController_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdcController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a76e4b0_a896_4219_86b6_ea8cdce98f56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ResolutionInBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ChannelMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdcChannelMode) -> ::windows::core::HRESULT,
    pub SetChannelMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AdcChannelMode) -> ::windows::core::HRESULT,
    pub IsChannelModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelmode: AdcChannelMode, result__: *mut bool) -> ::windows::core::HRESULT,
    pub OpenChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelnumber: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdcControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdcControllerStatics {
    type Vtable = IAdcControllerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdcControllerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcce98e0c_01f8_4891_bc3b_be53ef279ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Adc_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdcControllerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAdcControllerStatics2 {
    type Vtable = IAdcControllerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAdcControllerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2b93b1d_977b_4f5a_a5fe_a6abaffe6484);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdcControllerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc = "*Required features: `\"Devices_Adc\"`*"]
#[repr(transparent)]
pub struct AdcChannel(::windows::core::IUnknown);
impl AdcChannel {
    pub fn Controller(&self) -> ::windows::core::Result<AdcController> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Controller)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AdcController>(result__)
        }
    }
    pub fn ReadValue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ReadRatio(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadRatio)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for AdcChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdcChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdcChannel {}
impl ::core::fmt::Debug for AdcChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdcChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdcChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Adc.AdcChannel;{040bf414-2588-4a56-abef-73a260acc60a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AdcChannel {
    type Vtable = IAdcChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for AdcChannel {
    const IID: ::windows::core::GUID = <IAdcChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdcChannel {
    const NAME: &'static str = "Windows.Devices.Adc.AdcChannel";
}
::windows::core::interface_hierarchy!(AdcChannel, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AdcChannel> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AdcChannel) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AdcChannel> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AdcChannel) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&AdcChannel> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &AdcChannel) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for AdcChannel {}
unsafe impl ::core::marker::Sync for AdcChannel {}
#[doc = "*Required features: `\"Devices_Adc\"`*"]
#[repr(transparent)]
pub struct AdcController(::windows::core::IUnknown);
impl AdcController {
    pub fn ChannelCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChannelCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ResolutionInBits(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResolutionInBits)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MinValue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MinValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn MaxValue(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ChannelMode(&self) -> ::windows::core::Result<AdcChannelMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChannelMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<AdcChannelMode>(result__)
        }
    }
    pub fn SetChannelMode(&self, value: AdcChannelMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetChannelMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsChannelModeSupported(&self, channelmode: AdcChannelMode) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsChannelModeSupported)(::windows::core::Vtable::as_raw(this), channelmode, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn OpenChannel(&self, channelnumber: i32) -> ::windows::core::Result<AdcChannel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenChannel)(::windows::core::Vtable::as_raw(this), channelnumber, result__.as_mut_ptr()).from_abi::<AdcChannel>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Adc_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<'a, P0, E0>(provider: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AdcController>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, Provider::IAdcProvider>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IAdcControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetControllersAsync)(::windows::core::Vtable::as_raw(this), provider.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AdcController>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdcController>> {
        Self::IAdcControllerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<AdcController>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAdcControllerStatics<R, F: FnOnce(&IAdcControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AdcController, IAdcControllerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAdcControllerStatics2<R, F: FnOnce(&IAdcControllerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AdcController, IAdcControllerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AdcController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdcController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdcController {}
impl ::core::fmt::Debug for AdcController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdcController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdcController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Adc.AdcController;{2a76e4b0-a896-4219-86b6-ea8cdce98f56})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AdcController {
    type Vtable = IAdcController_Vtbl;
}
unsafe impl ::windows::core::Interface for AdcController {
    const IID: ::windows::core::GUID = <IAdcController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdcController {
    const NAME: &'static str = "Windows.Devices.Adc.AdcController";
}
::windows::core::interface_hierarchy!(AdcController, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AdcController {}
unsafe impl ::core::marker::Sync for AdcController {}
#[doc = "*Required features: `\"Devices_Adc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AdcChannelMode(pub i32);
impl AdcChannelMode {
    pub const SingleEnded: Self = Self(0i32);
    pub const Differential: Self = Self(1i32);
}
impl ::core::marker::Copy for AdcChannelMode {}
impl ::core::clone::Clone for AdcChannelMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdcChannelMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdcChannelMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdcChannelMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdcChannelMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdcChannelMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Adc.AdcChannelMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
