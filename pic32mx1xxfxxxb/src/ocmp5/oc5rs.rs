#[doc = "Reader of register OC5RS"]
pub type R = crate::R<u32, super::OC5RS>;
#[doc = "Writer for register OC5RS"]
pub type W = crate::W<u32, super::OC5RS>;
#[doc = "Register OC5RS `reset()`'s with value 0"]
impl crate::ResetValue for super::OC5RS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OC5RS`"]
pub type OC5RS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OC5RS`"]
pub struct OC5RS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5RS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc5rs(&self) -> OC5RS_R {
        OC5RS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc5rs(&mut self) -> OC5RS_W {
        OC5RS_W { w: self }
    }
}
