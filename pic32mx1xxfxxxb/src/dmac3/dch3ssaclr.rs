#[doc = "Reader of register DCH3SSACLR"]
pub type R = crate::R<u32, super::DCH3SSACLR>;
#[doc = "Writer for register DCH3SSACLR"]
pub type W = crate::W<u32, super::DCH3SSACLR>;
#[doc = "Register DCH3SSACLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH3SSACLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCH3SSA`"]
pub type DCH3SSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCH3SSA`"]
pub struct DCH3SSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCH3SSA_W<'a> {
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
    pub fn dch3ssa(&self) -> DCH3SSA_R {
        DCH3SSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dch3ssa(&mut self) -> DCH3SSA_W {
        DCH3SSA_W { w: self }
    }
}
