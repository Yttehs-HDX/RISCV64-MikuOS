/* 39 bits Virtual Address
 *
 *  38  30 29  21 20  12 11          0
 * +------+------+------+-------------+
 * | VPN2 | VPN1 | VPN0 | Page Offset |
 * +------+------+------+-------------+
 *    9      9      9         12
 *
 */

use crate::config::{PAGE_OFFSET, PAGE_SIZE};

pub const SV39_VPN_NUM: usize = 3;
pub const SV39_PER_VPN_WIDTH: usize = 9;
pub const SV39_VPN_WIDTH: usize = SV39_PER_VPN_WIDTH * SV39_VPN_NUM; // 27
pub const SV39_VA_WIDTH: usize = SV39_VPN_WIDTH + PAGE_OFFSET; // 39

// region VirtAddr begin
pub struct VirtAddr(pub usize);

impl VirtAddr {
    pub fn page_offset(&self) -> usize { self.0 & (PAGE_SIZE - 1) }

    pub fn aligned(&self) -> bool { self.page_offset() == 0 }

    pub fn floor(&self) -> VirtAddr { VirtAddr(self.0 & !(PAGE_SIZE - 1)) }

    pub fn floor_page(&self) -> VirtPageNum { self.floor().vpn() }

    pub fn ceil(&self) -> VirtAddr { VirtAddr((self.0 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) }

    pub fn ceil_page(&self) -> VirtPageNum { self.ceil().vpn() }

    pub fn vpn(&self) -> VirtPageNum { VirtPageNum(self.0 / PAGE_SIZE) }
}
// region VirtAddr end

// region VirtPageNum begin
pub struct VirtPageNum(pub usize);

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; SV39_VPN_NUM] {
        let mut vpn = self.0;
        let mut indexes = [0usize; SV39_VPN_NUM];
        indexes.iter_mut().rev().for_each( |idx| {
            *idx = vpn & ((1 << SV39_PER_VPN_WIDTH) - 1);
            vpn >>= SV39_PER_VPN_WIDTH;
        });
        indexes
    }

    pub fn va(&self) -> VirtAddr { VirtAddr(self.0 << PAGE_OFFSET) }
}
// region VirtPageNum end