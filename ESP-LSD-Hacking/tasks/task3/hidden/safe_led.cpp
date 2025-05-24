#include "../accessable/safe_led.h"
#include "internal.h"
#include <stdlib.h>

uint8_t size;
uint8_t limit_width;
uint8_t limit_height;

void create_safe_led(int led_count, uint8_t limit_width_, uint8_t limit_height_) {
    uint8_t leds_size = led_count * sizeof(Colorcode);
    size = led_count;
    limit_height = limit_height_;
    limit_width = limit_width_;
}

void set_led(void* led_object, uint8_t x, uint8_t y, Colorcode colorcode) {
    if (limit_height < y || limit_width < x)
        return;
    leds[x + 8 * y] = colorcode;
}

Colorcode* get_led_arr(void* led_object) {
    return leds;
}