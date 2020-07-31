// This file is handmade
//
// The original file was not usable as it had compiler errors. Creating a new file and copying the typedefs over was easier
// We're not interested in the stm32f2xx stuff anyway

#define __IO volatile

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef uint32_t size_t;

typedef struct
{
    __IO uint32_t MODER;   /*!< GPIO port mode register,               Address offset: 0x00      */
    __IO uint32_t OTYPER;  /*!< GPIO port output type register,        Address offset: 0x04      */
    __IO uint32_t OSPEEDR; /*!< GPIO port output speed register,       Address offset: 0x08      */
    __IO uint32_t PUPDR;   /*!< GPIO port pull-up/pull-down register,  Address offset: 0x0C      */
    __IO uint32_t IDR;     /*!< GPIO port input data register,         Address offset: 0x10      */
    __IO uint32_t ODR;     /*!< GPIO port output data register,        Address offset: 0x14      */
    __IO uint16_t BSRRL;   /*!< GPIO port bit set/reset low register,  Address offset: 0x18      */
    __IO uint16_t BSRRH;   /*!< GPIO port bit set/reset high register, Address offset: 0x1A      */
    __IO uint32_t LCKR;    /*!< GPIO port configuration lock register, Address offset: 0x1C      */
    __IO uint32_t AFR[2];  /*!< GPIO alternate function registers,     Address offset: 0x24-0x28 */
} GPIO_TypeDef;