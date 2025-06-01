mmdisasm is a simple disassembler  (currently for x86 32-bit, 64-bit, and ARM64) that includes basic uses of nm, hexdump, and objdump.

## hexdump

```bash
matteo@rust$ ./mmdisasm -f /etc/profile
00000000: 23 20 2f 65 74 63 2f 70 72 6f 66 69 6c 65 3a |# /etc/profile:|
0000000f: 20 73 79 73 74 65 6d 2d 77 69 64 65 20 2e 70 | system-wide .p|
0000001e: 72 6f 66 69 6c 65 20 66 69 6c 65 20 66 6f 72 |rofile file for|
0000002d: 20 74 68 65 20 42 6f 75 72 6e 65 20 73 68 65 | the Bourne she|
0000003c: 6c 6c 20 28 73 68 28 31 29 29 0a 23 20 61 6e |ll (sh(1)).# an|
0000004b: 64 20 42 6f 75 72 6e 65 20 63 6f 6d 70 61 74 |d Bourne compat|
0000005a: 69 62 6c 65 20 73 68 65 6c 6c 73 20 28 62 61 |ible shells (ba|
00000069: 73 68 28 31 29 2c 20 6b 73 68 28 31 29 2c 20 |sh(1), ksh(1), |
00000078: 61 73 68 28 31 29 2c 20 2e 2e 2e 29 2e 0a 0a |ash(1), ...)...|
00000087: 69 66 20 5b 20 22 24 28 69 64 20 2d 75 29 22 |if [ "$(id -u)"|
...
```

## disassemble a binary file (ELF, Mach-O)

```bash
matteo@rust$ ./mmdisasm -f /tmp/a.out -d 
Disassembling '.text' section [Arch: X86_64]

0x00001050 <_start>:
0x00001050:	xor	ebp, ebp
0x00001052:	mov	r9, rdx
0x00001055:	pop	rsi
0x00001056:	mov	rdx, rsp
0x00001059:	and	rsp, 0xfffffffffffffff0
0x0000105d:	push	rax
0x0000105e:	push	rsp
0x0000105f:	xor	r8d, r8d
0x00001062:	xor	ecx, ecx
0x00001064:	lea	rdi, [rip + 0xce]
0x0000106b:	call	qword ptr [rip + 0x2f4f]
0x00001071:	hlt
0x00001072:	nop	word ptr cs:[rax + rax]
0x0000107c:	nop	dword ptr [rax]

...
```

AT&T syntax for x86:
```bash
matteo@rust$ ./mmdisasm -f /tmp/a.out -d -a
Disassembling '.init' section [Arch: X86_64]

0x00001000 <_init>:
0x00001000:	subq	$8, %rsp
0x00001004:	movq	0x2fc5(%rip), %rax
0x0000100b:	testq	%rax, %rax
0x0000100e:	je	0x1012
0x00001010:	callq	*%rax
0x00001012:	addq	$8, %rsp
0x00001016:	retq

Disassembling '.plt' section [Arch: X86_64]
0x00001020:	pushq	0x2fca(%rip)
0x00001026:	jmpq	*0x2fcc(%rip)
0x0000102c:	nopl	(%rax)
0x00001030:	jmpq	*0x2fca(%rip)
0x00001036:	pushq	$0
0x0000103b:	jmp	0x1020

Disassembling '.plt.got' section [Arch: X86_64]
0x00001040:	jmpq	*0x2f9a(%rip)
0x00001046:	nop
..
```

##  nm
```bash
matteo@rust$ ./mdisasm -f /tmp/a.out -n
0000000000000000 Scrt1.o
000000000000037c __abi_tag
0000000000000000 crtstuff.c
0000000000001080 deregister_tm_clones
00000000000010b0 register_tm_clones
00000000000010f0 __do_global_dtors_aux
0000000000004018 completed.0
0000000000003dd8 __do_global_dtors_aux_fini_array_entry
0000000000001130 frame_dummy
0000000000003dd0 __frame_dummy_init_array_entry
0000000000000000 1.c
0000000000000000 crtstuff.c
00000000000020e0 __FRAME_END__
0000000000000000
0000000000003de0 _DYNAMIC
000000000000200c __GNU_EH_FRAME_HDR
0000000000003fe8 _GLOBAL_OFFSET_TABLE_
undef __libc_start_main@GLIBC_2.34
undef _ITM_deregisterTMCloneTable
0000000000004008 data_start
0000000000004018 _edata
0000000000001158 _fini
undef printf@GLIBC_2.2.5
0000000000004008 __data_start
undef __gmon_start__
0000000000004010 __dso_handle
0000000000002000 _IO_stdin_used
0000000000004020 _end
0000000000001050 _start
0000000000004018 __bss_start
0000000000001139 main
0000000000004018 __TMC_END__
undef _ITM_registerTMCloneTable
undef __cxa_finalize@GLIBC_2.2.5
0000000000001000 _init
```

##  print binary sections
```bash
matteo@rust$ ./mdisasm -f /tmp/a.out -s
section .interp
section .note.gnu.property
section .note.gnu.build-id
section .note.ABI-tag
section .gnu.hash
section .dynsym
section .dynstr
section .gnu.version
section .gnu.version_r
section .rela.dyn
section .rela.plt
section .init
section .plt
section .plt.got
section .text
section .fini
section .rodata
section .eh_frame_hdr
section .eh_frame
section .init_array
section .fini_array
section .dynamic
section .got
section .got.plt
section .data
section .bss
section .comment
section .symtab
section .strtab
section .shstrtab
```
##  section dump
```bash
matteo@rust:/tmp/disasm/target/debug$ ./mdisasm -f /tmp/a.out -S .comment
Dumping '.comment' section
00000000: 47 43 43 3a 20 28 44 65 62 69 61 6e 20 31 32 |GCC: (Debian 12|
0000000f: 2e 32 2e 30 2d 31 34 2b 64 65 62 31 32 75 31 |.2.0-14+deb12u1|
0000001e: 29 20 31 32 2e 32 2e 30 00                   |) 12.2.0.|
```

and so on...
