#[doc = "Reader of register DCH2CPTRSET"]
pub type R = crate::R<u32, super::DCH2CPTRSET>;
#[doc = "Writer for register DCH2CPTRSET"]
pub type W = crate::W<u32, super::DCH2CPTRSET>;
#[doc = "Register DCH2CPTRSET `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH2CPTRSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHCPTR`"]
pub type CHCPTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHCPTR`"]
pub struct CHCPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chcptr(&self) -> CHCPTR_R {
        CHCPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chcptr(&mut self) -> CHCPTR_W {
        CHCPTR_W { w: self }
    }
}