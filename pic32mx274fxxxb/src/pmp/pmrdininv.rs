#[doc = "Register `PMRDININV` reader"]
pub struct R(crate::R<PMRDININV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMRDININV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMRDININV_SPEC>> for R {
    fn from(reader: crate::R<PMRDININV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMRDININV` writer"]
pub struct W(crate::W<PMRDININV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMRDININV_SPEC>;
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
impl core::convert::From<crate::W<PMRDININV_SPEC>> for W {
    fn from(writer: crate::W<PMRDININV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDATAIN` reader - "]
pub struct RDATAIN_R(crate::FieldReader<u32, u32>);
impl RDATAIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        RDATAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATAIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDATAIN` writer - "]
pub struct RDATAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDATAIN_W<'a> {
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
    pub fn rdatain(&self) -> RDATAIN_R {
        RDATAIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rdatain(&mut self) -> RDATAIN_W {
        RDATAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMRDININV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmrdininv](index.html) module"]
pub struct PMRDININV_SPEC;
impl crate::RegisterSpec for PMRDININV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmrdininv::R](R) reader structure"]
impl crate::Readable for PMRDININV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmrdininv::W](W) writer structure"]
impl crate::Writable for PMRDININV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMRDININV to value 0"]
impl crate::Resettable for PMRDININV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
