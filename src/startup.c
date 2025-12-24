/**
 * Startup code for TM4C123GH6PM
 */

#include <stdint.h>

/* Linker symbols */
extern uint32_t _estack;
extern uint32_t _sdata;
extern uint32_t _edata;
extern uint32_t _sidata;
extern uint32_t _sbss;
extern uint32_t _ebss;

/* Main function prototype */
extern int main(void);

/* Exception handlers */
void Reset_Handler(void);
void Default_Handler(void);

/* Weak aliases for exception handlers */
void NMI_Handler(void)          __attribute__((weak, alias("Default_Handler")));
void HardFault_Handler(void)    __attribute__((weak, alias("Default_Handler")));
void MemManage_Handler(void)    __attribute__((weak, alias("Default_Handler")));
void BusFault_Handler(void)     __attribute__((weak, alias("Default_Handler")));
void UsageFault_Handler(void)   __attribute__((weak, alias("Default_Handler")));
void SVC_Handler(void)          __attribute__((weak, alias("Default_Handler")));
void DebugMon_Handler(void)     __attribute__((weak, alias("Default_Handler")));
void PendSV_Handler(void)       __attribute__((weak, alias("Default_Handler")));
void SysTick_Handler(void)      __attribute__((weak, alias("Default_Handler")));

/* Vector table */
__attribute__((section(".isr_vector")))
void (* const vector_table[])(void) = {
    (void (*)(void))(&_estack),  /* Initial Stack Pointer */
    Reset_Handler,               /* Reset Handler */
    NMI_Handler,                 /* NMI Handler */
    HardFault_Handler,           /* Hard Fault Handler */
    MemManage_Handler,           /* MPU Fault Handler */
    BusFault_Handler,            /* Bus Fault Handler */
    UsageFault_Handler,          /* Usage Fault Handler */
    0,                           /* Reserved */
    0,                           /* Reserved */
    0,                           /* Reserved */
    0,                           /* Reserved */
    SVC_Handler,                 /* SVCall Handler */
    DebugMon_Handler,            /* Debug Monitor Handler */
    0,                           /* Reserved */
    PendSV_Handler,              /* PendSV Handler */
    SysTick_Handler,             /* SysTick Handler */
    /* External Interrupts - TM4C123GH6PM has many more, add as needed */
};

/**
 * Reset Handler - Entry point after reset
 */
void Reset_Handler(void) {
    uint32_t *src, *dst;
    
    /* Copy .data section from Flash to SRAM */
    src = &_sidata;
    dst = &_sdata;
    while (dst < &_edata) {
        *dst++ = *src++;
    }
    
    /* Zero fill .bss section */
    dst = &_sbss;
    while (dst < &_ebss) {
        *dst++ = 0;
    }
    
    /* Call main() */
    main();
    
    /* Infinite loop if main returns */
    while (1) {}
}

/**
 * Default Handler for unimplemented interrupts
 */
void Default_Handler(void) {
    while (1) {}
}

