#include "secret.h"
#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SoftwareSerial.h>
#include <string>

#define wait_for(c) while(!(c)) yield();

void blink(void);
void handle_not_found(void);
void handle_command(void);

// serial connection
const byte txPin = D7;
const byte rxPin = D1;

// control signals
const byte to_gabby = D6;
const byte from_gabby = D2;

// status LED
const byte led = D3;

WiFiServer server(80);
SoftwareSerial gabby_serial(rxPin, txPin, true);

void setup() {
    // serial connection
    pinMode(rxPin, INPUT);
    pinMode(txPin, OUTPUT);

    // control signals
    pinMode(to_gabby, OUTPUT);
    digitalWrite(to_gabby, HIGH);
    pinMode(from_gabby, INPUT);

    // LED
    pinMode(led, OUTPUT);
    digitalWrite(led, LOW);

    blink();

    Serial.begin(115200);
    gabby_serial.begin(4800);

    WiFi.begin(WIFI_NAME, WIFI_PW);
    Serial.println("");
    Serial.print("Connecting");
    blink();
    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print(".");
    }
    Serial.println();
    Serial.print("Connected, IP address: ");
    Serial.println(WiFi.localIP());

    
    server.begin();

    blink();
}

void loop() {
    WiFiClient client = server.available();
    if (client) {
        blink();
        while (client.connected()) {
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
    digitalWrite(led, HIGH);
    delay(250);
    digitalWrite(led, LOW);
    delay(250);
}
