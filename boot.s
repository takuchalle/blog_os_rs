.option norvc

.section .boot, "ax", @progbits

.global _boot
.global trap_entry

_boot:
	la	sp, stack + 1024
	j	_start

trap_entry:
	j	_main

.bss

stack:
	.skip 1024

