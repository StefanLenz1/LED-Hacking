#include <stdint.h>

// Basic primary colors
#define COLOR_RED     0xFF0000  // Red: R=255, G=0, B=0
#define COLOR_GREEN   0x00FF00  // Green: R=0, G=255, B=0
#define COLOR_BLUE    0x0000FF  // Blue: R=0, G=0, B=255

// Secondary colors (combinations)
#define COLOR_CYAN    0x00FFFF  // Cyan: G + B
#define COLOR_MAGENTA 0xFF00FF  // Magenta: R + B
#define COLOR_YELLOW  0xFFFF00  // Yellow: R + G

// White, black, gray
#define COLOR_WHITE   0xFFFFFF  // White: R + G + B
#define COLOR_BLACK   0x000000  // Black: none
#define COLOR_GRAY    0x808080  // Gray: R=G=B=128

// Some extras
#define COLOR_ORANGE  0xFFA500  // Orange
#define COLOR_PURPLE  0x800080  // Purple
#define COLOR_PINK    0xFFC0CB  // Pink
#define COLOR_BROWN   0x8B4513  // Brown

void set_led(void* led_object, uint8_t x, uint8_t y, uint32_t colorcode);