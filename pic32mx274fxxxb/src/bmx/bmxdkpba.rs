#[doc = "Register `BMXDKPBA` reader"]
pub struct R(crate::R<BMXDKPBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMXDKPBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMXDKPBA_SPEC>> for R {
    fn from(reader: crate::R<BMXDKPBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMXDKPBA` writer"]
pub struct W(crate::W<BMXDKPBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMXDKPBA_SPEC>;
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
impl core::convert::From<crate::W<BMXDKPBA_SPEC>> for W {
    fn from(writer: crate::W<BMXDKPBA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMXDKPBA` reader - "]
pub struct BMXDKPBA_R(crate::FieldReader<u32, u32>);
impl BMXDKPBA_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMXDKPBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXDKPBA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXDKPBA` writer - "]
pub struct BMXDKPBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXDKPBA_W<'a> {
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
    pub fn bmxdkpba(&self) -> BMXDKPBA_R {
        BMXDKPBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxdkpba(&mut self) -> BMXDKPBA_W {
        BMXDKPBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMXDKPBA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdkpba](index.html) module"]
pub struct BMXDKPBA_SPEC;
impl crate::RegisterSpec for BMXDKPBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmxdkpba::R](R) reader structure"]
impl crate::Readable for BMXDKPBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmxdkpba::W](W) writer structure"]
impl crate::Writable for BMXDKPBA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMXDKPBA to value 0"]
impl crate::Resettable for BMXDKPBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
