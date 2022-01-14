#include "secret.h"
#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SoftwareSerial.h>
#include <string>

#define wait_for(c) while(!(c)) yield();

const byte txPin = D7;
const byte rxPin = D1;
const byte to_gabby = D6;
const byte from_gabby = D2;
const byte led = D3;


ESP8266WebServer server(80);
SoftwareSerial gabby_serial(rxPin, txPin, true);

void blink(void);
void handle_root(void);
void handle_not_found(void);
void serial_send(void);


void setup() {
    // LED
    pinMode(led, OUTPUT);
    digitalWrite(led, LOW);

    // UART
    pinMode(rxPin, INPUT);
    pinMode(txPin, OUTPUT);

    // handshake
    pinMode(to_gabby, OUTPUT);
    digitalWrite(to_gabby, HIGH);
    pinMode(from_gabby, INPUT);

    blink();

    Serial.begin(115200);
    gabby_serial.begin(4800);

    WiFi.begin(WIFI_NAME, WIFI_PW);
    Serial.print("Connecting");
    blink();
    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print(".");
    }
    Serial.println();
    Serial.print("Connected, IP address: ");
    Serial.println(WiFi.localIP());
    blink();

    server.on("/", handle_root);
    server.on("/SerialSend", serial_send);
    server.onNotFound(handle_not_found);
    server.begin();

}

void loop() {
    server.handleClient();
}

void blink(void) {
    digitalWrite(led, HIGH);
    delay(250);
    digitalWrite(led, LOW);
    delay(250);
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

void handle_root(void) {
    server.send(200, "text/plain", "Hello world!");
    blink();
}

void handle_not_found(void) {
    server.send(404, "text/plain", "404: Not found");
    blink();
    blink();
}

void serial_send(void) {
    if (!server.hasArg("control") || !server.hasArg("data")) {
        server.send(400, "text/plain", "invalid request");
        return;
    }
    String control = server.arg("control");
    String data = server.arg("data");

    if (control.indexOf("on") >= 0)
        switch_online();

    if (data != NULL && !data.isEmpty()) {
        byte buf[256];
        data.getBytes(buf, 256);
        int len = data.length() > 256 ? 256 : data.length();
        len = (len / 4) * 4;
        byte commands[128];
        for (int i = 0; i < len; i += 2) {
            int a = (buf[i] >= 'A') ? (buf[i] - 'A' + 10) : (buf[i] - '0');
            a <<= 4;
            a |= (buf[i+1] >= 'A') ? (buf[i+1] - 'A' + 10) : (buf[i+1] - '0');
            commands[i/2] = a;
        }
        if (len/2 > 0) {
            Serial.print("bleh: ");
            Serial.println(bytes_to_hex(commands, len/2));
            send_bytes(commands, len/2);
        }
    }

    if (control.indexOf("off") >= 0)
        switch_offline();

    blink();
    server.send(200, "text/plain", "Sent data");
}

void switch_online(void) {
    byte data[] = {0xA0, 0x00, 0xA1, 0x00, 0xA4, 0x00, 0xA2, 0x00};
    send_bytes(data, 8);
}

void switch_offline(void) {
    byte data[] = {0xA3, 0x00, 0xA0, 0x00};
    send_bytes(data, 4);
}

void send_bytes(byte data[], byte count) {
    for (byte i = 0; i < count; i+=2) {
        String msg = send_command(data[i], data[i+1]);
        Serial.print("response: ");
        Serial.println(msg);
    }
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
    } else
        return String();
}