#[doc = "Reader of register ADDCLR"]
pub type R = crate::R<u32, super::ADDCLR>;
#[doc = "Writer for register ADDCLR"]
pub type W = crate::W<u32, super::ADDCLR>;
#[doc = "Register ADDCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2CADD`"]
pub type I2CADD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2CADD`"]
pub struct I2CADD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2cadd(&self) -> I2CADD_R {
        I2CADD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn i2cadd(&mut self) -> I2CADD_W {
        I2CADD_W { w: self }
    }
}