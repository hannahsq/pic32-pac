#[doc = "Reader of register DSA"]
pub type R = crate::R<u32, super::DSA>;
#[doc = "Writer for register DSA"]
pub type W = crate::W<u32, super::DSA>;
#[doc = "Register DSA `reset()`'s with value 0"]
impl crate::ResetValue for super::DSA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSA`"]
pub type DSA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSA`"]
pub struct DSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DSA_W<'a> {
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
    pub fn dsa(&self) -> DSA_R {
        DSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dsa(&mut self) -> DSA_W {
        DSA_W { w: self }
    }
}