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

ESP8266WebServer server(80);
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

    server.on("/", handle_command);
    server.onNotFound(handle_not_found);
    server.begin();

    blink();
}

void loop() {
    server.handleClient();
}

//* util
void blink(void) {
    digitalWrite(led, HIGH);
    delay(250);
    digitalWrite(led, LOW);
    delay(250);
}

byte hex_val(char c) {
    return (c >= 'A') ? (c - 'A' + 10) : (c - '0');
}

char nyble(byte value) {
    if (value <= 9)
        return '0' + value;
    else
        return 'A' + value - 10;
}

String bytes_to_hex(byte *buf, int count) {
    String out = String();
    for (int i = 0; i < count; i++) {
        out += nyble(buf[i] >> 4);
        out += nyble(buf[i] & 0xF);
        out += " ";
    }
    return out;
}

//* handlers
void handle_not_found(void) {
    server.send(404, "text/plain", "404: Not found");
    blink();
    blink();
}

void handle_command(void) {
    if (!server.hasArg("data")
        || server.arg("data") == NULL
        || server.arg("data").isEmpty()) {

        server.send(400, "text/plain", "invalid request");
        return;
    }
    String data = server.arg("data");

    int data_len = (data.length() / 4) * 4;
    if (data_len > 10000 || data_len == 0) {
        server.send(400, "text/plain", "command size over 10k chars");
        return;
    }
    int len = data_len / 2;

    String commands = String();
    commands.reserve(len);
    for (int i = 0; i < data_len; i += 2) {
        commands = (hex_val(data.charAt(i)) << 4) 
                        | hex_val(data.charAt(i+1));
    }
    //Serial.print("sent commands: ");
    //Serial.println(bytes_to_hex(commands, len));

    for (byte i = 0; i < len; i+=2) {
        send_command(data[i], data[i+1]);
        //Serial.print("response: ");
        //Serial.println(msg);
    }

    blink();
    server.send(200, "text/plain", "Sent data");
}

String send_command(byte first, byte second) {
    gabby_serial.write(first);
    wait_for(digitalRead(from_gabby) == LOW);
    wait_for(digitalRead(from_gabby) == HIGH);
    gabby_serial.write(second);
    wait_for(digitalRead(from_gabby) == LOW);
    wait_for(digitalRead(from_gabby) == HIGH);
    if ((first & 0xF0) == 0xA0) {
        wait_for(gabby_serial.available());
        byte buf[128];
        int i = 1;
        buf[0] = gabby_serial.read();
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
        return bytes_to_hex(buf, i);
    }
}