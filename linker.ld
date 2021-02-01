OUTPUT_ARCH("riscv")

MEMORY {
	RAM(rwx) : ORIGIN = 0x80000000, LENGTH = 32K
}

ENTRY(_boot)

SECTIONS {
    .text :
	{
		*(.boot)
		*(.text .text.*)
	} > RAM

    /* rodata: Read-only data */
    .rodata :
	{
		*(.rdata .rodata .rodata.*)
	} > RAM

    /* data: Writable data */
    .data :
	{
		*(.data .data.*)
	} > RAM

    .bss :
	{
		*(.bss bss.*)
	} > RAM
	
	.comment : { *(.comment)  } > RAM
	.riscv : {
		*(.riscv.*)
	} > RAM
	
	.eh_frame : {
		*(.eh_frame)
	} > RAM
}
