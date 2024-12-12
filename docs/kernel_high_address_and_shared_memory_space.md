# 内核高地址与用户共页

## 内核高地址

### 目的

内核高地址加载是实现内核用户共页的前提。通常编译器会把用户的地址设置在低地址，但是 SBI 最后会将 pc 寄存器跳转到 0x8020_0000，也属于低地址的范围，使用内核高地址定义与低地址加载可以有效解决这个问题。

### 实现

使用 rust-objdump -d 读取内核 elf 文件的输出：

```txt
file format elf64-littleriscv

Sections:
Idx Name              Size     VMA              LMA              Type
  0                   00000000 0000000000000000 0000000000000000 
  1 .text             000254a6 ffffffffc0200000 0000000080200000 TEXT
  2 .rodata           0001e44f ffffffffc0226000 0000000080226000 DATA
  3 .data             00001010 ffffffffc0245000 0000000080245000 DATA
  4 .bss              01201218 ffffffffc0247000 0000000080247000 BSS
  ... (省略其他 Sections)
```

[linker.ld](../kernel/src/linker.ld#L15) 文件定义内核的 `VMA` 为高地址，使用 **AT** 定义 `LMA` 为低地址，于是在 QEMU 启动内核时，内核被加载到 0x8020_0000 物理地址，但是内核中定义的符号的起始位置从 0xffff_ffff_c020_0000 开始。

这时不能直接访问任何符号的地址，否则会发生 LoadPageFault。

riscv64 存在 MMU 元件，在进入内核后，立即在临时根页表（.data.root_page）将 0xffff_ffff_c000_0000 >> 12 和 0x8000_0000 >> 12 两个大小为 1 GB 的虚拟巨页映射到 0x8000_0000 的物理地址，写入 satp 寄存器启用 SV39 分页模式，此时内核以一级页表的方式完全被映射到高地址，以 0xffff_ffff_c020_0000 为起始位置的符号可以被正常访问。

跳转 pc 寄存器到达高地址的 rust_main 函数，之后内核不再使用低地址。内核初始化 [mm](../kernel/src/mm/memory_set/mod.rs#L99) 模块，重新构建三级页表映射，临时根页表被弃用。

> Q: 为什么临时根页表需要对 0x8000_0000 的低地址进行 identity map ？  
> A: 由于 pc 寄存器使用偏移量（offset）跳转，需要保证每时每刻 pc 寄存器储存的地址是有效的。

之后有一个细节值得注意，现在内核的地址空间映射模式是偏移映射，写入 PageTableEntry 时，要对 PPN 减去偏移量，保证 PPN 段是低地址，从 PageTableEntry 读取 PPN 后，要对 PPN 加上偏移量，获得可用的高地址 PPN。在修改 PhysPageNum 内容的函数中，使用 assert! 宏断言 PPN 处于高地址。

## 内核用户共页

### 目的

用户程序使用 ecall 发生中断，内核会调用对应的系统调用处理函数，大部分系统调用的参数包含指针类型，内核需要对指针解引用读取或者写入数据。内核与用户使用同一个页表后，内核可以利用 MMU 单元来读写指针，不需要手动查询页表，简化了系统调用处理的代码实现。

### 实现

由于内核与用户共享页表，要保证内核与用户的地址没有交集。

模块 [config](../kernel/src/config/mm.rs#L14) 定义 TrapContext 对象、UserStack、KernelStack 以及 free ppn range 的地址从 0xffff_ffff_ffff_ffff（usize::MAX + 1） 向下排列。而用户程序通常处于 0x10000 起始的低地址，两者不会引起冲突。
