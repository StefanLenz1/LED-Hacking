#include "../accessable/safe_led.hpp"
#include "internal.h"
#include <stdlib.h>

safe_led::safe_led(int led_count, uint8_t limit_width_, uint8_t limit_height_) {
    uint8_t struct_size = sizeof(safe_led);
    uint8_t leds_size = led_count * sizeof(Colorcode);
    this->size = led_count;
    this->limit_height = limit_height_;
    this->limit_width = limit_width_;
    this->leds;
    for (int i = 0; i < 64; ++i) {
        leds[i].components.r = 0;
        leds[i].components.g = 0;
        leds[i].components.b = 0;
    }
}

void safe_led::set_led(uint8_t x, uint8_t y, Colorcode colorcode) {
    if (limit_height < y || limit_width < x)
        return;
    if (limit_height < y || limit_width < x)
        return;
    leds[x + 8 * y] = colorcode;
}