/* SV39 Virtual Address: 39 bits
 *
 * |39    31|30    22|21    12|11     0|
 * | VPN[2] | VPN[1] | VPN[0] | Offset |
 * |--------|--------|--------|--------|
 * |   9    |   9    |   9    |   12   |
 *
 * | <---- VirtPageNum -----> | 27 bits
 * | <---- VirtAddr -----------------> | 39 bits
 */

use crate::{
    config::{SV39_OFFSET_BITS, SV39_PAGE_SIZE},
    util::StepByOne,
};

pub const SV39_VA_BITS: usize = 39;
pub const SV39_VPN_BITS: usize = 27;
const SV39_VPN_NUM: usize = 3;
const SV39_VPN_PER_BITS: usize = SV39_VPN_BITS / SV39_VPN_NUM;

// region VirtAddr begin
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);

impl VirtAddr {
    pub fn page_offset(&self) -> usize {
        self.0 & (SV39_PAGE_SIZE - 1)
    }
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }

    pub fn to_vpn(&self) -> VirtPageNum {
        assert!(self.aligned());
        VirtPageNum(self.0 >> SV39_OFFSET_BITS)
    }
    pub fn to_vpn_floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 >> SV39_OFFSET_BITS)
    }
    pub fn to_vpn_ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 + SV39_PAGE_SIZE - 1) >> SV39_OFFSET_BITS)
    }
}
// region VirtAddr end

// region VirtPageNum begin
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtPageNum(pub usize);

impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; SV39_VPN_NUM] {
        let mut vpn = self.0;
        let mut idxs = [0usize; SV39_VPN_NUM];
        for i in (0..SV39_VPN_NUM).rev() {
            idxs[i] = vpn & SV39_VPN_PER_BITS;
            vpn >>= SV39_VPN_PER_BITS;
        }
        idxs
    }
}
// region VirtPageNum end
