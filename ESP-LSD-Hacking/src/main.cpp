#include <Arduino.h>
#include <Adafruit_NeoPixel.h>

#define USE_TASK1

#ifdef USE_TASK1
#include "task1/hidden/internal.h"
#include "task1/accessable/usercode.h"
#include "task1/accessable/safe_led.h"
#endif

#ifdef USE_TASK2
#define INTERNAL_ACCESS
#include "task2/hidden/internal.h"
#include "task2/accessable/usercode.h"
#include "task2/accessable/safe_led.hpp"
#endif

#define PIN 6
#define NUMPIXELS 64

#define LIMIT_WIDTH 6
#define LIMIT_HEIGHT 7

Adafruit_NeoPixel pixels(NUMPIXELS, PIN, NEO_GRB + NEO_KHZ800);

#ifdef USE_TASK2
safe_led led_handler;
#endif
#ifdef USE_TASK1
void* led_handler;
#endif

void setup() {
  Serial.begin(9600);
  delay(1000);
#ifdef USE_TASK2
led_handler = safe_led(NUM_LEDS, LIMIT_WIDTH - 1, LIMIT_HEIGHT - 1);
#endif
#ifdef USE_TASK1
led_handler = create_safe_led(NUM_LEDS, LIMIT_WIDTH - 1, LIMIT_HEIGHT - 1);
#endif
  pixels.begin();
}

void dump_bytes(void* ptr, size_t count) {
  char* p = (char*)ptr;
  for (size_t i = 0; i < count; i++) {
    Serial.print(i);
    Serial.print(": 0x");
    Serial.println((uint8_t)p[i], HEX);
  }
}

void loop() {
    pixels.clear();
    dump_bytes(&led_handler, 400);
    make_pattern(led_handler);
    
#ifdef USE_TASK2
Colorcode* led_array = led_handler.get_led_arr();
#endif
#ifdef USE_TASK1
Colorcode* led_array = get_led_arr(led_handler);
#endif
    for (int i = 0; i < 64; i++) {
      pixels.setPixelColor(i, pixels.Color(led_array[i].components.r, led_array[i].components.g, led_array[i].components.b));
    }

    pixels.show();
    delay(50);
}
