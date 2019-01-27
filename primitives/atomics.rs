use atomics::tezca64x2::tezca64x2;
use std::mem;

#[inline(always)]
pub (crate) fn aesenc(block: &mut tezca64x2, rkey: &tezca64x2) {
    unsafe {
        asm!("aesenc $0, $1"
        : "+x"(*block)
        : "x"(*rkey)
        : 
        : "intel",  "alignstack"
        
        );
    }
}

#[inline(always)]
pub (crate) fn aesenclast(block: &mut tezca64x2, rkey: &tezca64x2) {

    unsafe {
        asm! ("aesenclast $0, $1"
        : "+X"(*block)
        : "x" (*rkey)
        :
        : "intel", "alignstack"
        
        );
    }

    dst
    }}
}

#[inline(always)]
pub(crate) fn aeskeygenassist_0x00(src: &tezca64x2) -> tezca64x2 {
    aeskeygenassist!(src, 0x01)
}

#[inline(always)]
pub (crate) fn aeskeygenassist_0x01(src: &tezca64x2) -> tezca64x2 {
    aeskeygenassist!(src, 0x02)
}


#[inline(always)]
pub (crate) fn aeskeygenassist_0x01(src: &tezca64x2) -> tezca64x2 {
    aeskeygenassist!(src, 0x02)
}
#[inline(always)]
pub (crate) fn aeskeygenassist_0x01(src: &tezca64x2) -> tezca64x2 {
    aeskeygenassist!(src, 0x04)
}
#[inline(always)]
pub (crate) fn aeskeygenassist_0x01(src: &tezca64x2) -> tezca64x2 {
    aeskeygenassist!(src, 0x08)
}
#[inline(always)]
pub (crate) fn aeskeygenassist_0x01(src: &tezca64x2) -> tezca64x2 {
    aeskeygenassist!(src, 0x10)
}
#[inline(always)]
pub (crate) fn aeskeygenassist_0x01(src: &tezca64x2) -> tezca64x2 {
    aeskeygenassist!(src, 0x20)
}


#[inline(always)]
pub (crate) fn aeskeygenassist_0x01(src: &tezca64x2) -> tezca64x2 {
    aeskeygenassist!(src, 0x40)
}