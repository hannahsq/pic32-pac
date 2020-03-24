#[doc = "Reader of register DCH1DSACLR"]
pub type R = crate::R<u32, super::DCH1DSACLR>;
#[doc = "Writer for register DCH1DSACLR"]
pub type W = crate::W<u32, super::DCH1DSACLR>;
#[doc = "Register DCH1DSACLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH1DSACLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCH1DSA`"]
pub type DCH1DSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCH1DSA`"]
pub struct DCH1DSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCH1DSA_W<'a> {
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
    pub fn dch1dsa(&self) -> DCH1DSA_R {
        DCH1DSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dch1dsa(&mut self) -> DCH1DSA_W {
        DCH1DSA_W { w: self }
    }
}
