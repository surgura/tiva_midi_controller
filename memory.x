/* Memory layout for TM4C123GH6PM */
/* 256KB Flash, 32KB SRAM */

MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 256K
    RAM   : ORIGIN = 0x20000000, LENGTH = 32K
}

/* Stack size - reduced from 8KB to test if stack overflow is causing issues */
_stack_size = 0x1000;  /* 4KB stack */

/* TivaWare IntRegister() requires a RAM vector table (.vtable section)
 * that is 1KB-aligned and placed at the start of SRAM.
 * This allows TivaWare to dynamically register interrupt handlers.
 */
SECTIONS
{
    /* Place .vtable section at start of RAM, 1KB-aligned */
    .vtable (NOLOAD) : ALIGN(1024)
    {
        . = ALIGN(1024);
        *(.vtable)
        . = ALIGN(1024);
    } > RAM
}

