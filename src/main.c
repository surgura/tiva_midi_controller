/**
 * Minimal Tiva C Series TM4C123GXL LaunchPad Example
 * Blinks the onboard RGB LED (Red on PF1)
 */

#include <stdint.h>

/* Register definitions for TM4C123GH6PM */
#define SYSCTL_RCGCGPIO     (*((volatile uint32_t *)0x400FE608))
#define SYSCTL_PRGPIO       (*((volatile uint32_t *)0x400FEA08))

#define GPIO_PORTF_BASE     0x40025000
#define GPIO_PORTF_DATA     (*((volatile uint32_t *)(GPIO_PORTF_BASE + 0x3FC)))
#define GPIO_PORTF_DIR      (*((volatile uint32_t *)(GPIO_PORTF_BASE + 0x400)))
#define GPIO_PORTF_DEN      (*((volatile uint32_t *)(GPIO_PORTF_BASE + 0x51C)))

/* LED pins on Port F */
#define LED_RED     (1 << 1)  /* PF1 */
#define LED_BLUE    (1 << 2)  /* PF2 */
#define LED_GREEN   (1 << 3)  /* PF3 */

/* Simple delay function */
static void delay(volatile uint32_t count) {
    while (count--) {
        __asm__ volatile ("nop");
    }
}

int main(void) {
    /* Enable clock for GPIO Port F */
    SYSCTL_RCGCGPIO |= (1 << 5);  /* Port F is bit 5 */
    
    /* Wait for peripheral to be ready */
    while ((SYSCTL_PRGPIO & (1 << 5)) == 0) {}
    
    /* Configure PF1, PF2, PF3 as outputs (RGB LED) */
    GPIO_PORTF_DIR |= (LED_RED | LED_BLUE | LED_GREEN);
    
    /* Enable digital function for the LED pins */
    GPIO_PORTF_DEN |= (LED_RED | LED_BLUE | LED_GREEN);
    
    /* Main loop - blink red LED */
    while (1) {
        GPIO_PORTF_DATA ^= LED_RED;  /* Toggle red LED */
        delay(500000);
    }
    
    return 0;
}

