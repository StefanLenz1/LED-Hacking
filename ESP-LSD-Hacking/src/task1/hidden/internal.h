#include "../accessable/safe_led.h"

Colorcode* get_led_arr(void* led_object);
void* create_safe_led(int led_count, uint8_t limit_width, uint8_t limit_height);
void destroy_safe_led(void* led_object);
void set_all(void* led_object, Colorcode colorcode);