macro_rules! declare_optional {
    ($ty:ty) => {
        pub mod optional {
            pub fn static_variant_type() -> std::borrow::Cow<'static, $crate::glib::VariantTy> {
                let mut builder = $crate::glib::GStringBuilder::new("m");
                builder.append(super::static_variant_type().as_str());
                std::borrow::Cow::Owned(
                    $crate::glib::VariantType::from_string(builder.into_string()).unwrap(),
                )
            }
            pub fn to_variant(value: &Option<$ty>) -> $crate::glib::Variant {
                match value.as_ref() {
                    Some(value) => $crate::glib::Variant::from_some(&super::to_variant(value)),
                    None => $crate::glib::Variant::from_none(&*super::static_variant_type()),
                }
            }
            pub fn from_variant(variant: &$crate::glib::Variant) -> Option<Option<$ty>> {
                if !variant.is_type(&*static_variant_type()) {
                    return None;
                }
                match variant.as_maybe() {
                    Some(variant) => Some(Some(super::from_variant(&variant)?)),
                    None => Some(None),
                }
            }
        }
    };
}

#[cfg(feature = "use_cairo")]
pub mod cairo;
#[cfg(feature = "use_gdk4")]
pub mod gdk4;
#[cfg(feature = "use_gio")]
pub mod gio;
pub mod glib;
#[cfg(feature = "use_graphene")]
pub mod graphene;
#[cfg(feature = "use_gtk4")]
pub mod gtk4;

use ::glib::ffi;

struct VariantBuilder(ffi::GVariantBuilder);

#[allow(dead_code)]
impl VariantBuilder {
    fn new(ty: &::glib::VariantTy) -> Self {
        use ::glib::translate::ToGlibPtr;
        let mut builder: std::mem::MaybeUninit<ffi::GVariantBuilder> =
            std::mem::MaybeUninit::uninit();
        Self(unsafe {
            ffi::g_variant_builder_init(builder.as_mut_ptr(), ty.to_glib_none().0);
            builder.assume_init()
        })
    }
    fn end(self) -> ::glib::Variant {
        let v = unsafe { self.end_unsafe() };
        std::mem::forget(self);
        v
    }
}

impl Drop for VariantBuilder {
    fn drop(&mut self) {
        unsafe { ffi::g_variant_builder_clear(self.as_ptr()) };
    }
}

trait VariantBuilderExt {
    fn as_ptr(&self) -> *mut ffi::GVariantBuilder;
    unsafe fn add<T: ::glib::ToVariant>(&self, value: &T) {
        self.add_value(&value.to_variant());
    }
    unsafe fn add_value(&self, value: &::glib::Variant) {
        use ::glib::translate::ToGlibPtr;
        ffi::g_variant_builder_add_value(self.as_ptr(), value.to_glib_none().0);
    }
    fn open(&self, ty: &::glib::VariantTy) -> VariantBuilderContainer<'_> {
        use ::glib::translate::ToGlibPtr;
        unsafe { ffi::g_variant_builder_open(self.as_ptr(), ty.to_glib_none().0) };
        VariantBuilderContainer {
            inner: std::ptr::NonNull::new(self.as_ptr()).unwrap(),
            phantom: std::marker::PhantomData,
        }
    }
    unsafe fn end_unsafe(&self) -> ::glib::Variant {
        ::glib::translate::from_glib_none(ffi::g_variant_builder_end(self.as_ptr()))
    }
}

impl VariantBuilderExt for VariantBuilder {
    fn as_ptr(&self) -> *mut ffi::GVariantBuilder {
        &self.0 as *const _ as *mut _
    }
}

#[repr(transparent)]
struct VariantBuilderContainer<'t> {
    inner: std::ptr::NonNull<ffi::GVariantBuilder>,
    phantom: std::marker::PhantomData<&'t ()>,
}

impl<'t> Drop for VariantBuilderContainer<'t> {
    fn drop(&mut self) {
        unsafe { ffi::g_variant_builder_close(self.inner.as_ptr()) };
    }
}

impl<'t> VariantBuilderExt for VariantBuilderContainer<'t> {
    fn as_ptr(&self) -> *mut ffi::GVariantBuilder {
        self.inner.as_ptr()
    }
}
