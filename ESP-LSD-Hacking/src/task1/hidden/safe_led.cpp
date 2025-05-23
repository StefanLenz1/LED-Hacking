#include "../accessable/safe_led.h"
#include "internal.h"
#include <stdlib.h>

struct safe_led {
    uint8_t size;
    Colorcode leds[64];
    uint8_t limit_width;
    uint8_t limit_height;
};

void* create_safe_led(int led_count, uint8_t limit_width_, uint8_t limit_height_) {
    uint8_t struct_size = sizeof(safe_led);
    uint8_t leds_size = led_count * sizeof(Colorcode);
    safe_led* new_struct = (safe_led*) malloc(struct_size);
    safe_led zero_struct = {0};
    *new_struct = zero_struct;
    new_struct->size = led_count;
    new_struct->limit_height = limit_height_;
    new_struct->limit_width = limit_width_;
    return new_struct;
}

void destroy_safe_led(void* led_object) {
    free(led_object);
}

void set_led(void* led_object, uint8_t x, uint8_t y, Colorcode colorcode) {
    safe_led* led_object_casted = (safe_led*) led_object;
    if (led_object_casted->limit_height < y || led_object_casted->limit_width < x)
        return;
    if (led_object_casted->limit_height < y || led_object_casted->limit_width < x)
        return;
    led_object_casted->leds[x + 8 * y] = colorcode;
}

void set_all(void* led_object, Colorcode colorcode) {
    safe_led* led_object_casted = (safe_led*) led_object;
    for (int i = 0; i < 64; i++) {
        led_object_casted->leds[i] = colorcode;
    }
}

Colorcode* get_led_arr(void* led_object) {
    safe_led* led_object_casted = (safe_led*) led_object;
    return led_object_casted->leds;
}

bool color_equal(Colorcode a, Colorcode b) {
    return a.value == b.value;
}