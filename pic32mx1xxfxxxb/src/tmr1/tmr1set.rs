#[doc = "Reader of register TMR1SET"]
pub type R = crate::R<u32, super::TMR1SET>;
#[doc = "Writer for register TMR1SET"]
pub type W = crate::W<u32, super::TMR1SET>;
#[doc = "Register TMR1SET `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR1SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMR1`"]
pub type TMR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TMR1`"]
pub struct TMR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1_W<'a> {
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
    pub fn tmr1(&self) -> TMR1_R {
        TMR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr1(&mut self) -> TMR1_W {
        TMR1_W { w: self }
    }
}
