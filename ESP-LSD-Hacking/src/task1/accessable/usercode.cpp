#include "usercode.h"
#include "safe_led.h"
#include "Arduino.h"

void make_pattern(void* led_object) {
    set_led(led_object, 0, 0, COLOR_WHITE);
    set_led(led_object, 1, 0, COLOR_RED);
    set_led(led_object, 2, 0, COLOR_GREEN);
    set_led(led_object, 2, 2, COLOR_RED);
    set_led(led_object, 6, 6, COLOR_BLUE);
    Colorcode get_cell = get_led(led_object, 5, 5);
    if (color_equal(get_cell, COLOR_BLUE))
        set_led(led_object, 5, 5, COLOR_BLUE);
    else
        set_led(led_object, 5, 5, COLOR_RED);
}