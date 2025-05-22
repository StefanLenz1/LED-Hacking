#include <FastLED.h>
#include "safe_led.h"

struct safe_led {
    size_t size;
    CRGB* leds;
};

void* create_safe_led(int led_count) {
    size_t struct_size = sizeof(safe_led);
    size_t leds_size = led_count * sizeof(CRGB);
    safe_led* new_struct = (safe_led*) malloc(struct_size + leds_size);
    new_struct->size = led_count;
    new_struct->leds = (CRGB*)((uint8_t*)new_struct + struct_size);
    memset(new_struct->leds, 0, leds_size);
    return new_struct;
}

void destroy_safe_led(void* led_object) {
    free(led_object);
}

void set_led(void* led_object, uint8_t x, uint8_t y, uint32_t colorcode) {
    safe_led* led_object_casted = (safe_led*) led_object;
    int edge_length = (int)sqrt(led_object_casted->size);
    led_object_casted->leds[x + edge_length * y] = CRGB(colorcode);
}