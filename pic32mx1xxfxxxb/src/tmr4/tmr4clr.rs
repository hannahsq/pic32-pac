#[doc = "Register `TMR4CLR` reader"]
pub struct R(crate::R<TMR4CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR4CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TMR4CLR_SPEC>> for R {
    fn from(reader: crate::R<TMR4CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR4CLR` writer"]
pub struct W(crate::W<TMR4CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR4CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<TMR4CLR_SPEC>> for W {
    fn from(writer: crate::W<TMR4CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR4` reader - "]
pub struct TMR4_R(crate::FieldReader<u32, u32>);
impl TMR4_R {
    pub(crate) fn new(bits: u32) -> Self {
        TMR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR4` writer - "]
pub struct TMR4_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR4_W<'a> {
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
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr4(&mut self) -> TMR4_W {
        TMR4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR4CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr4clr](index.html) module"]
pub struct TMR4CLR_SPEC;
impl crate::RegisterSpec for TMR4CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr4clr::R](R) reader structure"]
impl crate::Readable for TMR4CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr4clr::W](W) writer structure"]
impl crate::Writable for TMR4CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR4CLR to value 0"]
impl crate::Resettable for TMR4CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
