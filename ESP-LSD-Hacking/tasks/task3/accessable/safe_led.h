#ifndef SAFE_LED
#define SAFE_LED

#include <stdint.h>
#include <stdbool.h>

typedef union {
    uint32_t value;
    struct {
        uint8_t b;
        uint8_t g;
        uint8_t r;
        uint8_t a;  // padding
    } components;
} Colorcode;

const Colorcode COLOR_RED     = { .value = 0x00FF0000 };
const Colorcode COLOR_GREEN   = { .value = 0x0000FF00 };
const Colorcode COLOR_BLUE    = { .value = 0x000000FF };

const Colorcode COLOR_CYAN    = { .value = 0x0000FFFF };
const Colorcode COLOR_MAGENTA = { .value = 0x00FF00FF };
const Colorcode COLOR_YELLOW  = { .value = 0x00FFFF00 };

const Colorcode COLOR_WHITE   = { .value = 0x00FFFFFF };
const Colorcode COLOR_BLACK   = { .value = 0x00000000 };
const Colorcode COLOR_GRAY    = { .value = 0x00808080 };

const Colorcode COLOR_ORANGE  = { .value = 0x00FFA500 };
const Colorcode COLOR_PURPLE  = { .value = 0x00800080 };
const Colorcode COLOR_PINK    = { .value = 0x00FFC0CB };
const Colorcode COLOR_BROWN   = { .value = 0x008B4513 };

void set_led(uint8_t x, uint8_t y, Colorcode colorcode);
bool color_equal(Colorcode a, Colorcode b);

static Colorcode leds[64] = {};

#endif