#include <FastLED.h>
#include "../accessable/safe_led.h"
#include "internal.h"

struct safe_led {
    size_t size;
    CRGB leds[64];
    uint8_t limit_width;
    uint8_t limit_height;
};

void* create_safe_led(int led_count, uint8_t limit_width_, uint8_t limit_height_) {
    size_t struct_size = sizeof(safe_led);
    size_t leds_size = led_count * sizeof(CRGB);
    safe_led* new_struct = (safe_led*) malloc(struct_size);
    new_struct->size = led_count;
    new_struct->limit_height = limit_height_;
    new_struct->limit_width = limit_width_;
    //new_struct->leds = (CRGB*)((uint8_t*)new_struct + struct_size);
    memset(new_struct->leds, 0, leds_size);
    return new_struct;
}

void destroy_safe_led(void* led_object) {
    free(led_object);
}

void set_led(void* led_object, uint8_t x, uint8_t y, Colorcode colorcode) {
    safe_led* led_object_casted = (safe_led*) led_object;
    if (led_object_casted->limit_height < y || led_object_casted->limit_width < x)
        return;
    int edge_length = (int)sqrt(led_object_casted->size);
    led_object_casted->leds[x + edge_length * y] = CRGB(colorcode.value);
}

Colorcode get_led(void* led_object, uint8_t x, uint8_t y) {
    safe_led* led_object_casted = (safe_led*) led_object;
    int edge_length = (int)sqrt(led_object_casted->size);
    auto led_to_unpack = led_object_casted->leds[x + edge_length * y];
    Colorcode newstruct = {};
    newstruct.components.r = led_to_unpack.red;
    newstruct.components.b = led_to_unpack.blue;
    newstruct.components.g = led_to_unpack.green;
    return newstruct;
}

CRGB* get_led_arr(void* led_object) {
    safe_led* led_object_casted = (safe_led*) led_object;
    return led_object_casted->leds;
}

bool color_equal(Colorcode a, Colorcode b) {
    return a.value == b.value;
}