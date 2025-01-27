#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialSurfaceInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8e9ebe7_39b7_3962_bb03_57f56e1fb0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UpdateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateTime: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryGetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryGetBounds: usize,
    #[cfg(feature = "Foundation")]
    pub TryComputeLatestMeshAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryComputeLatestMeshAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryComputeLatestMeshWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxtrianglespercubicmeter: f64, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryComputeLatestMeshWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMesh(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialSurfaceMesh {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x108f57d9_df0d_3950_a0fd_f972c77c27b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMesh_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SurfaceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TriangleIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VertexPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub VertexPositionScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    VertexPositionScale: usize,
    pub VertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshBuffer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93cf59e0_871f_33f8_98b2_03d101458f6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshBuffer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    pub Stride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ElementCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2759f89_3572_3d2d_a10d_5fee9394aa37);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexPositionFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexPositionFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexPositionFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub TriangleIndexFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    TriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetTriangleIndexFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetTriangleIndexFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub VertexNormalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    VertexNormalFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetVertexNormalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetVertexNormalFormat: usize,
    pub IncludeVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceMeshOptionsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialSurfaceMeshOptionsStatics {
    type Vtable = ISpatialSurfaceMeshOptionsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialSurfaceMeshOptionsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b340abf_9781_4505_8935_013575caae5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceMeshOptionsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexPositionFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexPositionFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedTriangleIndexFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedTriangleIndexFormats: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub SupportedVertexNormalFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))]
    SupportedVertexNormalFormats: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserver(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10b69819_ddca_3483_ac3a_748fe8c86df5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserver_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetObservedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetObservedSurfaces: usize,
    pub SetBoundingVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetBoundingVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetBoundingVolumes: usize,
    #[cfg(feature = "Foundation")]
    pub ObservedSurfacesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ObservedSurfacesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveObservedSurfacesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveObservedSurfacesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialSurfaceObserverStatics {
    type Vtable = ISpatialSurfaceObserverStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserverStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x165951ed_2108_4168_9175_87e027bc9285);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialSurfaceObserverStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISpatialSurfaceObserverStatics2 {
    type Vtable = ISpatialSurfaceObserverStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ISpatialSurfaceObserverStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f534261_c55d_4e6b_a895_a19de69a42e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialSurfaceObserverStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceInfo(::windows::core::IUnknown);
impl SpatialSurfaceInfo {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryGetBounds(&self, coordinatesystem: &super::SpatialCoordinateSystem) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetBounds)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(coordinatesystem), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::SpatialBoundingOrientedBox>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryComputeLatestMeshAsync(&self, maxtrianglespercubicmeter: f64) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryComputeLatestMeshAsync)(::windows::core::Vtable::as_raw(this), maxtrianglespercubicmeter, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryComputeLatestMeshWithOptionsAsync(&self, maxtrianglespercubicmeter: f64, options: &SpatialSurfaceMeshOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryComputeLatestMeshWithOptionsAsync)(::windows::core::Vtable::as_raw(this), maxtrianglespercubicmeter, ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<SpatialSurfaceMesh>>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceInfo {}
impl ::core::fmt::Debug for SpatialSurfaceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo;{f8e9ebe7-39b7-3962-bb03-57f56e1fb0a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SpatialSurfaceInfo {
    type Vtable = ISpatialSurfaceInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for SpatialSurfaceInfo {
    const IID: ::windows::core::GUID = <ISpatialSurfaceInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceInfo {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceInfo";
}
::windows::core::interface_hierarchy!(SpatialSurfaceInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceInfo {}
unsafe impl ::core::marker::Sync for SpatialSurfaceInfo {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMesh(::windows::core::IUnknown);
impl SpatialSurfaceMesh {
    pub fn SurfaceInfo(&self) -> ::windows::core::Result<SpatialSurfaceInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SurfaceInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialSurfaceInfo>(result__)
        }
    }
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CoordinateSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SpatialCoordinateSystem>(result__)
        }
    }
    pub fn TriangleIndices(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TriangleIndices)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
    pub fn VertexPositions(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VertexPositions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn VertexPositionScale(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VertexPositionScale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn VertexNormals(&self) -> ::windows::core::Result<SpatialSurfaceMeshBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VertexNormals)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<SpatialSurfaceMeshBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMesh {}
impl ::core::fmt::Debug for SpatialSurfaceMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMesh").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMesh {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh;{108f57d9-df0d-3950-a0fd-f972c77c27b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SpatialSurfaceMesh {
    type Vtable = ISpatialSurfaceMesh_Vtbl;
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMesh {
    const IID: ::windows::core::GUID = <ISpatialSurfaceMesh as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceMesh {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMesh";
}
::windows::core::interface_hierarchy!(SpatialSurfaceMesh, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceMesh {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMesh {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMeshBuffer(::windows::core::IUnknown);
impl SpatialSurfaceMeshBuffer {
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Format)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    pub fn Stride(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stride)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ElementCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ElementCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialSurfaceMeshBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMeshBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMeshBuffer {}
impl ::core::fmt::Debug for SpatialSurfaceMeshBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMeshBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMeshBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer;{93cf59e0-871f-33f8-98b2-03d101458f6f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SpatialSurfaceMeshBuffer {
    type Vtable = ISpatialSurfaceMeshBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMeshBuffer {
    const IID: ::windows::core::GUID = <ISpatialSurfaceMeshBuffer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceMeshBuffer {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshBuffer";
}
::windows::core::interface_hierarchy!(SpatialSurfaceMeshBuffer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceMeshBuffer {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshBuffer {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceMeshOptions(::windows::core::IUnknown);
impl SpatialSurfaceMeshOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpatialSurfaceMeshOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexPositionFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VertexPositionFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexPositionFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVertexPositionFormat)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn TriangleIndexFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TriangleIndexFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetTriangleIndexFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTriangleIndexFormat)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn VertexNormalFormat(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VertexNormalFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Graphics::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX\"`*"]
    #[cfg(feature = "Graphics_DirectX")]
    pub fn SetVertexNormalFormat(&self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVertexNormalFormat)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IncludeVertexNormals(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IncludeVertexNormals)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIncludeVertexNormals(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIncludeVertexNormals)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexPositionFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedVertexPositionFormats)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedTriangleIndexFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedTriangleIndexFormats)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_DirectX\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    pub fn SupportedVertexNormalFormats() -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>> {
        Self::ISpatialSurfaceMeshOptionsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedVertexNormalFormats)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Graphics::DirectX::DirectXPixelFormat>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceMeshOptionsStatics<R, F: FnOnce(&ISpatialSurfaceMeshOptionsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpatialSurfaceMeshOptions, ISpatialSurfaceMeshOptionsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SpatialSurfaceMeshOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceMeshOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceMeshOptions {}
impl ::core::fmt::Debug for SpatialSurfaceMeshOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceMeshOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceMeshOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions;{d2759f89-3572-3d2d-a10d-5fee9394aa37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SpatialSurfaceMeshOptions {
    type Vtable = ISpatialSurfaceMeshOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for SpatialSurfaceMeshOptions {
    const IID: ::windows::core::GUID = <ISpatialSurfaceMeshOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceMeshOptions {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceMeshOptions";
}
::windows::core::interface_hierarchy!(SpatialSurfaceMeshOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceMeshOptions {}
unsafe impl ::core::marker::Sync for SpatialSurfaceMeshOptions {}
#[doc = "*Required features: `\"Perception_Spatial_Surfaces\"`*"]
#[repr(transparent)]
pub struct SpatialSurfaceObserver(::windows::core::IUnknown);
impl SpatialSurfaceObserver {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetObservedSurfaces(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, SpatialSurfaceInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetObservedSurfaces)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, SpatialSurfaceInfo>>(result__)
        }
    }
    pub fn SetBoundingVolume(&self, bounds: &super::SpatialBoundingVolume) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBoundingVolume)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bounds)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetBoundingVolumes<'a, P0, E0>(&self, bounds: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<super::SpatialBoundingVolume>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBoundingVolumes)(::windows::core::Vtable::as_raw(this), bounds.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ObservedSurfacesChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<SpatialSurfaceObserver, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ObservedSurfacesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveObservedSurfacesChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveObservedSurfacesChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>> {
        Self::ISpatialSurfaceObserverStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>(result__)
        })
    }
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::ISpatialSurfaceObserverStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceObserverStatics<R, F: FnOnce(&ISpatialSurfaceObserverStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISpatialSurfaceObserverStatics2<R, F: FnOnce(&ISpatialSurfaceObserverStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SpatialSurfaceObserver, ISpatialSurfaceObserverStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SpatialSurfaceObserver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialSurfaceObserver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialSurfaceObserver {}
impl ::core::fmt::Debug for SpatialSurfaceObserver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialSurfaceObserver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialSurfaceObserver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver;{10b69819-ddca-3483-ac3a-748fe8c86df5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SpatialSurfaceObserver {
    type Vtable = ISpatialSurfaceObserver_Vtbl;
}
unsafe impl ::windows::core::Interface for SpatialSurfaceObserver {
    const IID: ::windows::core::GUID = <ISpatialSurfaceObserver as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialSurfaceObserver {
    const NAME: &'static str = "Windows.Perception.Spatial.Surfaces.SpatialSurfaceObserver";
}
::windows::core::interface_hierarchy!(SpatialSurfaceObserver, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SpatialSurfaceObserver {}
unsafe impl ::core::marker::Sync for SpatialSurfaceObserver {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
