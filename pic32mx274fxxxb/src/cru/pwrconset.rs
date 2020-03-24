#[doc = "Reader of register PWRCONSET"]
pub type R = crate::R<u32, super::PWRCONSET>;
#[doc = "Writer for register PWRCONSET"]
pub type W = crate::W<u32, super::PWRCONSET>;
#[doc = "Register PWRCONSET `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCONSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VREGS`"]
pub type VREGS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREGS`"]
pub struct VREGS_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vregs(&self) -> VREGS_R {
        VREGS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vregs(&mut self) -> VREGS_W {
        VREGS_W { w: self }
    }
}