#[doc = "Register `OC3RS` reader"]
pub struct R(crate::R<OC3RS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OC3RS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OC3RS_SPEC>> for R {
    fn from(reader: crate::R<OC3RS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OC3RS` writer"]
pub struct W(crate::W<OC3RS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OC3RS_SPEC>;
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
impl core::convert::From<crate::W<OC3RS_SPEC>> for W {
    fn from(writer: crate::W<OC3RS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC3RS` reader - "]
pub struct OC3RS_R(crate::FieldReader<u32, u32>);
impl OC3RS_R {
    pub(crate) fn new(bits: u32) -> Self {
        OC3RS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3RS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3RS` writer - "]
pub struct OC3RS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3RS_W<'a> {
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
    pub fn oc3rs(&self) -> OC3RS_R {
        OC3RS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc3rs(&mut self) -> OC3RS_W {
        OC3RS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OC3RS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3rs](index.html) module"]
pub struct OC3RS_SPEC;
impl crate::RegisterSpec for OC3RS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oc3rs::R](R) reader structure"]
impl crate::Readable for OC3RS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oc3rs::W](W) writer structure"]
impl crate::Writable for OC3RS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OC3RS to value 0"]
impl crate::Resettable for OC3RS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
