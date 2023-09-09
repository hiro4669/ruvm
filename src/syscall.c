#include <stdio.h>
#include <unistd.h>
#include <stdint.h>

void hello() {
    printf("Hello World!\n");
}

uint16_t sys_write(uint16_t fildes, uint8_t* buf, uint16_t len) {
    return write(fildes, buf, len);    
}

