use openexr_core_sys as sys;
use std::marker::PhantomData;

pub struct AttrString<'a> {
    _inner: openexr_core_sys::exr_attr_string_t,
    _data: &'a PhantomData<()>,
}

impl<'a> AttrString<'a> {
    pub fn str(&self) -> &'a str {
        todo!()
    }

    pub fn length(&self) -> usize {
        todo!()
    }

    pub fn alloc_size(&self) -> usize {
        todo!()
    }
}

impl<'a> AttrString<'a> {
    pub fn new<S: AsRef<str>>(_s: S) -> Self {
        todo!()
        // AttrString::Owned(CString::new(s.as_ref()).expect("Bad bytes in AttrString constructor"))
    }
}

impl<'a> From<&'a AttrString<'a>> for sys::exr_attr_string_t {
    fn from(_a: &'a AttrString) -> Self {
        todo!()
        // match a {
        //     AttrString::Ref(s) => sys::exr_attr_string_t {
        //         length: s.to_bytes_with_nul().len() as i32,
        //         alloc_size: 0,
        //         str_: s.as_ptr(),
        //     },
        //     AttrString::Owned(s) => sys::exr_attr_string_t {
        //         length: s.to_bytes_with_nul().len() as i32,
        //         alloc_size: 0,
        //         str_: s.as_ptr(),
        //     },
        // }
    }
}

impl<'a> From<&'a sys::exr_attr_string_t> for AttrString<'a> {
    fn from(_a: &'a sys::exr_attr_string_t) -> AttrString<'a> {
        todo!()
        // # Safety
        // This is safe as long as sys::exr_attr_string_t is null-terminated
        // unsafe { AttrString::Ref(CStr::from_ptr(a.str_)) }
    }
}
