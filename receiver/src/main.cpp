#include "secret.h"
#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SoftwareSerial.h>
#include <string>

#define wait_for(c) while(!(c)) yield();

void blink(void);

// serial connection
const byte txPin = D7;
const byte rxPin = D1;

// control signals
const byte to_gabby = D6;
const byte from_gabby = D2;

// status LED
const byte led = D4;

// keyboard
constexpr uint8_t keyboardSelect[5] = {1, 3, 14, 15, 16};
constexpr uint8_t keyboardIn = 0;

WiFiServer server(80);
SoftwareSerial gabby_serial(rxPin, txPin, true);

void setup() {
    pinMode(rxPin, INPUT);
    pinMode(txPin, OUTPUT);
    pinMode(to_gabby, OUTPUT);
    digitalWrite(to_gabby, HIGH);
    pinMode(from_gabby, INPUT);
    pinMode(led, OUTPUT);
    digitalWrite(led, HIGH);
    for (uint8_t pin : keyboardSelect) {
        pinMode(pin, OUTPUT);
        digitalWrite(pin, LOW);
    }
    pinMode(keyboardIn, INPUT);

    blink();
    
    blink();
    gabby_serial.begin(4800);
    WiFi.begin(WIFI_NAME, WIFI_PW);
    blink();
    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
    }
    server.begin();
    blink();
}


uint32_t scanKeys() {
    uint32_t data = 0;
    for (uint8_t keyboardWire = 0; keyboardWire < 32; keyboardWire++) {
        for (uint8_t pin = 0; pin < 5; pin++) {
            digitalWrite(keyboardSelect[pin], !!(keyboardWire & (1 << pin)));
        }
        data |= digitalRead(keyboardIn) << keyboardWire;
    }
    return data;
}


void loop() {
    WiFiClient client = server.accept();
    unsigned long time = 0;
    if (client) {
        blink();
        while (client.connected()) {
            if (millis() >= time + 500) {
                time = millis();
                client.write(0x02);
                uint32_t data = scanKeys();
                for (int8_t offset = 24; offset >= 0; offset -= 8) {
                    client.write(0xFF & (data >> offset));
                }
            }
            if (gabby_serial.available()) {
                byte data = gabby_serial.read();
                client.write(0x01);
                client.write(data);
            }
            if (client.available()) {
                byte first = client.read();
                byte second = client.read();
                gabby_serial.write(first);
                wait_for(digitalRead(from_gabby) == LOW);
                wait_for(digitalRead(from_gabby) == HIGH);
                gabby_serial.write(second);
                wait_for(digitalRead(from_gabby) == LOW);
                wait_for(digitalRead(from_gabby) == HIGH);
                byte buf[128];
                byte i = 0;
                if ((first & 0xF0) == 0xA0) {
                    wait_for(gabby_serial.available());
                    buf[0] = gabby_serial.read();
                    i++;
                    if (buf[0] == 0xA4) {
                        delayMicroseconds(2300);
                        while (gabby_serial.available()) {
                            buf[i] = gabby_serial.read();
                            i++;
                            delayMicroseconds(2300);
                        }
                    }
                    digitalWrite(to_gabby, LOW);
                    delay(1);
                    digitalWrite(to_gabby, HIGH);
                }
                client.write(i | 0x80);
                for (byte j = 0; j < i; j++) {
                    client.write(buf[j]);
                }
            }
        }
    }
}

//* util
void blink(void) {
    digitalWrite(led, LOW);
    delay(250);
    digitalWrite(led, HIGH);
    delay(250);
}
