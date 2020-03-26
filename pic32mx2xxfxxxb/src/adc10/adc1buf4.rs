#[doc = "Reader of register ADC1BUF4"]
pub type R = crate::R<u32, super::ADC1BUF4>;
#[doc = "Writer for register ADC1BUF4"]
pub type W = crate::W<u32, super::ADC1BUF4>;
#[doc = "Register ADC1BUF4 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC1BUF4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC1BUF4`"]
pub type ADC1BUF4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADC1BUF4`"]
pub struct ADC1BUF4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1BUF4_W<'a> {
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
    pub fn adc1buf4(&self) -> ADC1BUF4_R {
        ADC1BUF4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adc1buf4(&mut self) -> ADC1BUF4_W {
        ADC1BUF4_W { w: self }
    }
}
