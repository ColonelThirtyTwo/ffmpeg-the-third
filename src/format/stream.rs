use crate::codec;
use crate::ffi::*;
use crate::AsPtr;
use crate::Dictionary;
use crate::{DictionaryRef, Discard, Rational};

use std::ffi::c_int;

#[cfg(not(feature = "ffmpeg_8_0"))]
use crate::codec::packet;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct Disposition: c_int {
        const DEFAULT          = AV_DISPOSITION_DEFAULT;
        const DUB              = AV_DISPOSITION_DUB;
        const ORIGINAL         = AV_DISPOSITION_ORIGINAL;
        const COMMENT          = AV_DISPOSITION_COMMENT;
        const LYRICS           = AV_DISPOSITION_LYRICS;
        const KARAOKE          = AV_DISPOSITION_KARAOKE;
        const FORCED           = AV_DISPOSITION_FORCED;
        const HEARING_IMPAIRED = AV_DISPOSITION_HEARING_IMPAIRED;
        const VISUAL_IMPAIRED  = AV_DISPOSITION_VISUAL_IMPAIRED;
        const CLEAN_EFFECTS    = AV_DISPOSITION_CLEAN_EFFECTS;
        const ATTACHED_PIC     = AV_DISPOSITION_ATTACHED_PIC;
        const CAPTIONS         = AV_DISPOSITION_CAPTIONS;
        const DESCRIPTIONS     = AV_DISPOSITION_DESCRIPTIONS;
        const METADATA         = AV_DISPOSITION_METADATA;
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Stream(AVStream);

impl Stream {
    pub unsafe fn from_raw<'a>(ptr: *const AVStream) -> &'a Stream {
        let r = unsafe { &*ptr };
        // safety: this is a transparent wrapper
        std::mem::transmute::<&'a AVStream, &'a Self>(r)
    }

    pub unsafe fn from_raw_mut<'a>(ptr: *mut AVStream) -> &'a mut Stream {
        let r = unsafe { &mut *ptr };
        // safety: this is a transparent wrapper
        std::mem::transmute::<&'a mut AVStream, &'a mut Self>(r)
    }

    pub fn as_ptr(&self) -> *const AVStream {
        (self as *const Self).cast::<AVStream>()
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVStream {
        (self as *mut Self).cast::<AVStream>()
    }

    pub fn as_ref(&self) -> &AVStream {
        // safety: this is a transparent wrapper
        unsafe { std::mem::transmute::<&Self, &AVStream>(self) }
    }

    /// # Safety
    ///
    /// Fields must not be altered to invalid values
    pub unsafe fn as_mut(&mut self) -> &mut AVStream {
        // safety: this is a transparent wrapper
        unsafe { std::mem::transmute::<&mut Self, &mut AVStream>(self) }
    }
}

impl std::ops::Deref for Stream {
    type Target = AVStream;
    fn deref(&self) -> &Self::Target {
        // safety: this is a transparent wrapper
        unsafe { std::mem::transmute::<&Self, &AVStream>(self) }
    }
}

impl Stream {
    pub fn id(&self) -> i32 {
        self.as_ref().id
    }

    pub fn parameters(&self) -> codec::ParametersRef<'_> {
        unsafe {
            codec::ParametersRef::from_raw((*self.as_ptr()).codecpar).expect("codecpar is non-null")
        }
    }

    pub fn index(&self) -> usize {
        self.as_ref().index as usize
    }

    pub fn time_base(&self) -> Rational {
        Rational::from(self.as_ref().time_base)
    }

    pub fn start_time(&self) -> i64 {
        self.as_ref().start_time
    }

    pub fn duration(&self) -> i64 {
        self.as_ref().duration
    }

    pub fn frames(&self) -> i64 {
        self.as_ref().nb_frames
    }

    pub fn disposition(&self) -> Disposition {
        Disposition::from_bits_truncate(self.as_ref().disposition)
    }

    pub fn discard(&self) -> Discard {
        Discard::from(self.as_ref().discard)
    }

    #[cfg(not(feature = "ffmpeg_8_0"))]
    pub fn side_data(&self) -> SideDataIter<'_> {
        SideDataIter::new(self)
    }

    pub fn rate(&self) -> Rational {
        Rational::from(self.as_ref().r_frame_rate)
    }

    pub fn avg_frame_rate(&self) -> Rational {
        Rational::from(self.as_ref().avg_frame_rate)
    }

    pub fn metadata(&self) -> DictionaryRef<'_> {
        unsafe { DictionaryRef::wrap(self.as_ref().metadata) }
    }

    pub fn sample_aspect_ratio(&self) -> Rational {
        Rational::from(self.as_ref().sample_aspect_ratio)
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        unsafe { self.as_mut() }.time_base = value.into().into();
    }

    pub fn set_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe { self.as_mut() }.r_frame_rate = value.into().into();
    }

    pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        unsafe { self.as_mut() }.avg_frame_rate = value.into().into();
    }

    pub fn parameters_mut(&mut self) -> codec::ParametersMut<'_> {
        unsafe {
            codec::ParametersMut::from_raw(self.as_mut().codecpar).expect("codecpar is non-null")
        }
    }

    pub fn set_parameters<P: AsPtr<AVCodecParameters>>(&mut self, parameters: P) {
        unsafe {
            avcodec_parameters_copy(self.as_mut().codecpar, parameters.as_ptr());
        }
    }

    pub fn copy_parameters_from_context(&mut self, ctx: &codec::Context) {
        unsafe {
            avcodec_parameters_from_context(self.as_mut().codecpar, ctx.as_ptr());
        }
    }

    pub fn set_metadata(&mut self, metadata: Dictionary) {
        let metadata = metadata.disown();
        unsafe { self.as_mut() }.metadata = metadata;
    }

    pub fn set_sample_aspect_ratio(&mut self, sar: Rational) {
        unsafe { self.as_mut() }.sample_aspect_ratio = sar.into();
    }
}

#[cfg(not(feature = "ffmpeg_8_0"))]
#[derive(Clone, Copy)]
pub struct SideDataIter<'a, 'c> {
    stream: Stream<'a, 'c>,
    current: c_int,
}

#[cfg(not(feature = "ffmpeg_8_0"))]
impl<'a, 'c> SideDataIter<'a, 'c> {
    pub fn new(stream: Stream<'a, 'c>) -> SideDataIter<'a, 'c> {
        SideDataIter { stream, current: 0 }
    }
}

#[cfg(not(feature = "ffmpeg_8_0"))]
impl<'a, 'c> Iterator for SideDataIter<'a, 'c> {
    type Item = packet::SideData<'a, 'c>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            if self.current >= (*self.stream.as_ptr()).nb_side_data {
                return None;
            }

            self.current += 1;

            Some(packet::SideData::wrap(
                (*self.stream.as_ptr())
                    .side_data
                    .offset((self.current - 1) as isize),
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unsafe {
            let length = (*self.stream.as_ptr()).nb_side_data as usize;

            (
                length - self.current as usize,
                Some(length - self.current as usize),
            )
        }
    }
}

#[cfg(not(feature = "ffmpeg_8_0"))]
impl<'a, 'c> ExactSizeIterator for SideDataIter<'a, 'c> {}
