use crate::ffi::*;
use crate::{Dictionary, DictionaryMut, DictionaryRef, Rational};

#[derive(Debug)]
#[repr(transparent)]
pub struct Chapter(AVChapter);

impl Chapter {
    pub unsafe fn from_raw<'a>(ptr: *const AVChapter) -> &'a Chapter {
        let r = unsafe { &*ptr };
        // safety: this is a transparent wrapper
        std::mem::transmute::<&'a AVChapter, &'a Self>(r)
    }

    pub unsafe fn from_raw_mut<'a>(ptr: *mut AVChapter) -> &'a mut Chapter {
        let r = unsafe { &mut *ptr };
        // safety: this is a transparent wrapper
        std::mem::transmute::<&'a mut AVChapter, &'a mut Self>(r)
    }

    pub fn as_ptr(&self) -> *const AVChapter {
        (self as *const Self).cast::<AVChapter>()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVChapter {
        (self as *mut Self).cast::<AVChapter>()
    }

    pub fn as_ref(&self) -> &AVChapter {
        // safety: this is a transparent wrapper
        unsafe { std::mem::transmute::<&Self, &AVChapter>(self) }
    }

    /// # Safety
    ///
    /// Fields must not be altered to invalid values
    pub unsafe fn as_mut(&mut self) -> &mut AVChapter {
        // safety: this is a transparent wrapper
        unsafe { std::mem::transmute::<&mut Self, &mut AVChapter>(self) }
    }

    pub fn id(&self) -> i64 {
        self.as_ref().id
    }

    pub fn time_base(&self) -> Rational {
        Rational::from(self.as_ref().time_base)
    }

    pub fn start(&self) -> i64 {
        self.as_ref().start
    }

    pub fn end(&self) -> i64 {
        self.as_ref().end
    }

    pub fn metadata(&self) -> DictionaryRef<'_> {
        unsafe { DictionaryRef::wrap(self.as_ref().metadata) }
    }

    pub fn set_id(&mut self, value: i64) {
        unsafe { self.as_mut() }.id = value;
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe { self.as_mut() }.time_base = value.into().into();
    }

    pub fn set_start(&mut self, value: i64) {
        unsafe { self.as_mut() }.start = value;
    }

    pub fn set_end(&mut self, value: i64) {
        unsafe { self.as_mut() }.end = value;
    }

    pub fn set_metadata<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        // dictionary.set() allocates the AVDictionary the first time a key/value is inserted
        // so we want to update the metadata dictionary afterwards
        unsafe {
            let mut dictionary = Dictionary::own(self.metadata_mut().as_mut_ptr());
            dictionary.set(key.as_ref(), value.as_ref());
            (*self.as_mut_ptr()).metadata = dictionary.disown();
        }
    }

    pub fn metadata_mut(&mut self) -> DictionaryMut<'_> {
        unsafe { DictionaryMut::wrap(self.as_mut().metadata) }
    }
}
