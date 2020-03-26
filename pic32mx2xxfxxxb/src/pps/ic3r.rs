#[doc = "Reader of register IC3R"]
pub type R = crate::R<u32, super::IC3R>;
#[doc = "Writer for register IC3R"]
pub type W = crate::W<u32, super::IC3R>;
#[doc = "Register IC3R `reset()`'s with value 0"]
impl crate::ResetValue for super::IC3R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IC3R`"]
pub type IC3R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IC3R`"]
pub struct IC3R_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ic3r(&self) -> IC3R_R {
        IC3R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ic3r(&mut self) -> IC3R_W {
        IC3R_W { w: self }
    }
}
