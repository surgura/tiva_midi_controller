/* Memory layout for TM4C123GH6PM */
/* 256KB Flash, 32KB SRAM */

MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 256K
    RAM   : ORIGIN = 0x20000000, LENGTH = 32K
}

/* Increase stack size for USB initialization (default is 1KB) */
_stack_size = 0x2000;  /* 8KB stack */

