#ifndef INTERNAL
#define INTERNAL

#include <FastLED.h>
CRGB* get_led_arr(void* led_object);
void* create_safe_led(int led_count, uint8_t limit_width, uint8_t limit_height);
void destroy_safe_led(void* led_object);

#endif