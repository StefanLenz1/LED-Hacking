#include "usercode.h"
#include "safe_led.h"
#include "Arduino.h"



void set_led_wrapper(void *led_object, int x, int y, Colorcode color) {
    set_led(led_object, x, y, color);
}

const uint64_t IMAGES[] = {
  0x0000000000000000,
  0x3c4299a581a5423c
};
const int IMAGES_LEN = sizeof(IMAGES)/8;


void display_image(uint64_t image, void* led_object) {
    for (int y = 0; y < 8; y++) {
        for (int x = 0; x < 8; x++) {
            int bit_index = y * 8 + x;
            if ((image >> bit_index) & 1) {
                set_led_wrapper(led_object, x, y, COLOR_RED);
            } else {
                set_led_wrapper(led_object, x, y, COLOR_BLACK);
            }
        }
    }
}

void image_loop(void* led_object) {
    static int i = 0;
    display_image(IMAGES[i%2], led_object);
    i++;
}

void make_pattern(void* led_object) {

    image_loop(led_object);

    set_led_wrapper(led_object, 0, 0, COLOR_WHITE);
    set_led_wrapper(led_object, 0, 7, COLOR_BLUE);
    set_led_wrapper(led_object, 1, 1, COLOR_BLUE);
    set_led_wrapper(led_object, 7, 7, COLOR_BLUE);
    set_led_wrapper(led_object, 0, 1, COLOR_WHITE);
    set_led_wrapper(led_object, 1, 0, COLOR_WHITE);
}