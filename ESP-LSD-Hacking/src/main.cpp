#include <Arduino.h>
#include "task1/hidden/internal.h"
#include "task1/accessable/usercode.h"

#include <Adafruit_NeoPixel.h>
#include "task1/accessable/safe_led.h"

#define PIN 6
#define NUMPIXELS 64

#define LIMIT_WIDTH 6
#define LIMIT_HEIGHT 7

Adafruit_NeoPixel pixels(NUMPIXELS, PIN, NEO_GRB + NEO_KHZ800);

void* led_handler;

void setup() {
  Serial.begin(9600);
  delay(1000);
  led_handler = create_safe_led(NUM_LEDS, LIMIT_WIDTH - 1, LIMIT_HEIGHT - 1);
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
    dump_bytes(led_handler, 400);
    make_pattern(led_handler);
    Colorcode* led_array = get_led_arr(led_handler);
    //set_all(led_handler, COLOR_BLUE);
    
    for (int i = 0; i < 64; i++) {
      pixels.setPixelColor(i, pixels.Color(led_array[i].components.r, led_array[i].components.g, led_array[i].components.b));
    }

    pixels.show();
    delay(50);
}
