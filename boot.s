.option norvc

.section .boot, "ax", @progbits

.global _boot

_boot:
	la	sp, stack + 1024
	j	_start

.bss

stack:
	.skip 1024
