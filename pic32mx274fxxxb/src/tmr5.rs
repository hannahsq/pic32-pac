#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T5CON register"]
    pub t5con: crate::Reg<t5con::T5CON_SPEC>,
    #[doc = "0x04 - T5CONCLR register"]
    pub t5conclr: crate::Reg<t5conclr::T5CONCLR_SPEC>,
    #[doc = "0x08 - T5CONSET register"]
    pub t5conset: crate::Reg<t5conset::T5CONSET_SPEC>,
    #[doc = "0x0c - T5CONINV register"]
    pub t5coninv: crate::Reg<t5coninv::T5CONINV_SPEC>,
    #[doc = "0x10 - TMR5 register"]
    pub tmr5: crate::Reg<tmr5::TMR5_SPEC>,
    #[doc = "0x14 - TMR5CLR register"]
    pub tmr5clr: crate::Reg<tmr5clr::TMR5CLR_SPEC>,
    #[doc = "0x18 - TMR5SET register"]
    pub tmr5set: crate::Reg<tmr5set::TMR5SET_SPEC>,
    #[doc = "0x1c - TMR5INV register"]
    pub tmr5inv: crate::Reg<tmr5inv::TMR5INV_SPEC>,
    #[doc = "0x20 - PR5 register"]
    pub pr5: crate::Reg<pr5::PR5_SPEC>,
    #[doc = "0x24 - PR5CLR register"]
    pub pr5clr: crate::Reg<pr5clr::PR5CLR_SPEC>,
    #[doc = "0x28 - PR5SET register"]
    pub pr5set: crate::Reg<pr5set::PR5SET_SPEC>,
    #[doc = "0x2c - PR5INV register"]
    pub pr5inv: crate::Reg<pr5inv::PR5INV_SPEC>,
}
#[doc = "T5CON register accessor: an alias for `Reg<T5CON_SPEC>`"]
pub type T5CON = crate::Reg<t5con::T5CON_SPEC>;
#[doc = "T5CON register"]
pub mod t5con;
#[doc = "T5CONCLR register accessor: an alias for `Reg<T5CONCLR_SPEC>`"]
pub type T5CONCLR = crate::Reg<t5conclr::T5CONCLR_SPEC>;
#[doc = "T5CONCLR register"]
pub mod t5conclr;
#[doc = "T5CONSET register accessor: an alias for `Reg<T5CONSET_SPEC>`"]
pub type T5CONSET = crate::Reg<t5conset::T5CONSET_SPEC>;
#[doc = "T5CONSET register"]
pub mod t5conset;
#[doc = "T5CONINV register accessor: an alias for `Reg<T5CONINV_SPEC>`"]
pub type T5CONINV = crate::Reg<t5coninv::T5CONINV_SPEC>;
#[doc = "T5CONINV register"]
pub mod t5coninv;
#[doc = "TMR5 register accessor: an alias for `Reg<TMR5_SPEC>`"]
pub type TMR5 = crate::Reg<tmr5::TMR5_SPEC>;
#[doc = "TMR5 register"]
pub mod tmr5;
#[doc = "TMR5CLR register accessor: an alias for `Reg<TMR5CLR_SPEC>`"]
pub type TMR5CLR = crate::Reg<tmr5clr::TMR5CLR_SPEC>;
#[doc = "TMR5CLR register"]
pub mod tmr5clr;
#[doc = "TMR5SET register accessor: an alias for `Reg<TMR5SET_SPEC>`"]
pub type TMR5SET = crate::Reg<tmr5set::TMR5SET_SPEC>;
#[doc = "TMR5SET register"]
pub mod tmr5set;
#[doc = "TMR5INV register accessor: an alias for `Reg<TMR5INV_SPEC>`"]
pub type TMR5INV = crate::Reg<tmr5inv::TMR5INV_SPEC>;
#[doc = "TMR5INV register"]
pub mod tmr5inv;
#[doc = "PR5 register accessor: an alias for `Reg<PR5_SPEC>`"]
pub type PR5 = crate::Reg<pr5::PR5_SPEC>;
#[doc = "PR5 register"]
pub mod pr5;
#[doc = "PR5CLR register accessor: an alias for `Reg<PR5CLR_SPEC>`"]
pub type PR5CLR = crate::Reg<pr5clr::PR5CLR_SPEC>;
#[doc = "PR5CLR register"]
pub mod pr5clr;
#[doc = "PR5SET register accessor: an alias for `Reg<PR5SET_SPEC>`"]
pub type PR5SET = crate::Reg<pr5set::PR5SET_SPEC>;
#[doc = "PR5SET register"]
pub mod pr5set;
#[doc = "PR5INV register accessor: an alias for `Reg<PR5INV_SPEC>`"]
pub type PR5INV = crate::Reg<pr5inv::PR5INV_SPEC>;
#[doc = "PR5INV register"]
pub mod pr5inv;
