#[doc = "Reader of register RPB12R"]
pub type R = crate::R<u32, super::RPB12R>;
#[doc = "Writer for register RPB12R"]
pub type W = crate::W<u32, super::RPB12R>;
#[doc = "Register RPB12R `reset()`'s with value 0"]
impl crate::ResetValue for super::RPB12R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPB12R`"]
pub type RPB12R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPB12R`"]
pub struct RPB12R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPB12R_W<'a> {
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
    pub fn rpb12r(&self) -> RPB12R_R {
        RPB12R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpb12r(&mut self) -> RPB12R_W {
        RPB12R_W { w: self }
    }
}
