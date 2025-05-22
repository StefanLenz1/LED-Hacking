#include <Arduino.h>
#include <FastLED.h>

// LED matrix configuration
#define LED_PIN     12      // GPIO 12
#define NUM_LEDS    64      // 8x8 matrix
#define LED_TYPE    WS2812B
#define COLOR_ORDER GRB
#define BRIGHTNESS  64

CRGB leds[NUM_LEDS];

void setup() {
  delay(3000); // Power-up safety delay
  FastLED.addLeds<LED_TYPE, LED_PIN, COLOR_ORDER>(leds, NUM_LEDS);
  FastLED.setBrightness(BRIGHTNESS);
}

void loop() {
  // Simple rainbow animation
  static uint8_t hue = 0;
  for (int i = 0; i < NUM_LEDS; i++) {
    leds[i] = CHSV(hue + (i * 2), 255, 255);
  }
  FastLED.show();
  hue++;
  delay(50);
}