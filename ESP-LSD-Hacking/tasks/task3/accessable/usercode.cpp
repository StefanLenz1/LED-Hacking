#include "usercode.h"
#include "safe_led.h"
#include "Arduino.h"



void set_led_wrapper(int x, int y, Colorcode color) {
    //set_led(x, y, color);
    leds[x + y * 8] = color;
}

const uint64_t IMAGES[] = {
  0x0000000000000000,
  0x3c4299a581a5423c
};
const int IMAGES_LEN = sizeof(IMAGES)/8;


void display_image(uint64_t image) {
    for (int y = 0; y < 8; y++) {
        for (int x = 0; x < 8; x++) {
            int bit_index = y * 8 + x;
            if ((image >> bit_index) & 1) {
                set_led_wrapper(x, y, COLOR_RED);
            } else {
                set_led_wrapper(x, y, COLOR_BLACK);
            }
        }
    }
}

void image_loop() {
    static int i = 0;
    display_image(IMAGES[i%2]);
    i++;
}

void make_pattern() {

    image_loop();

    set_led_wrapper(0, 0, COLOR_WHITE);
    set_led_wrapper(0, 7, COLOR_BLUE);
    set_led_wrapper(1, 1, COLOR_BLUE);
    set_led_wrapper(7, 7, COLOR_BLUE);
    set_led_wrapper(0, 1, COLOR_WHITE);
    set_led_wrapper(1, 0, COLOR_WHITE);
}