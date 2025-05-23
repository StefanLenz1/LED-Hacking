#include <Arduino.h>
#include <FastLED.h>
#include "task1/hidden/internal.h"
#include "task1/accessable/usercode.h"

// LED matrix configuration
#define LED_PIN     12      // GPIO 12
#define NUM_LEDS    64      // 8x8 matrix
#define LED_TYPE    WS2812
#define COLOR_ORDER GRB
#define BRIGHTNESS  64

CRGB leds[NUM_LEDS];

void setup() {
  Serial.begin(9600);  // Start Serial at 9600 baud rate
  delay(3000); // Power-up safety delay
  FastLED.addLeds<LED_TYPE, LED_PIN, COLOR_ORDER>(leds, NUM_LEDS);
  FastLED.setBrightness(BRIGHTNESS);
}

void test();

void loop() {
  // Simple rainbow animation
  static uint8_t hue = 0;
  for (int i = 0; i < NUM_LEDS; i++) {
    leds[i] = CHSV(hue + (i * 2), 255, 255);
  }
  FastLED.show();
  hue++;
  delay(50);
  test();
}

void dump_bytes(void* ptr, size_t count) {
  char* p = (char*)ptr;
  for (size_t i = 0; i < count; i++) {
    Serial.print(i);
    Serial.print(": 0x");
    Serial.println((uint8_t)p[i], HEX);
  }
}

void test() {
  void* led_handler = create_safe_led(NUM_LEDS, 9, 9);
  while (true) {
    dump_bytes(led_handler, 200);
    make_pattern(led_handler);
    FastLED.show();
    delay(50);
  }
}