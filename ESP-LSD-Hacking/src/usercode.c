#include "safe_led.h"
#include "usercode.h"

void make_pattern(void* led_object) {
    set_led(led_object, 2, 2, COLOR_RED);
    set_led(led_object, 6, 6, COLOR_BLUE);
}