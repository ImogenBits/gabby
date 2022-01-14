#include "secret.h"
#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SoftwareSerial.h>
#include <string>

#define wait_for(c) while(!(c)) yield();

#define SERVER_PORT 80
const byte txPin = D7;
const byte rxPin = D1;
#define DELAY 1

const byte to_gabby = D6;
const byte from_gabby = D2;

const byte led = D3;


ESP8266WebServer server(80);

SoftwareSerial gabbySerial(rxPin, txPin, true);

void blink(void);

void handleRoot(void);
void handleNotFound(void);
void serialSend(void);

void switchOnline(void);
void switchOffline(void);
void sendCommand(const String &cmd);

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
    Serial.println();
    Serial.println("wah");

    gabbySerial.begin(4800);

    WiFi.begin(WIFI_NAME, WIFI_PW);
    Serial.print("Connecting");
    blink();
    while (WiFi.status() != WL_CONNECTED)
    {
    delay(500);
    Serial.print(".");
    }
    Serial.println();
    Serial.print("Connected, IP address: ");
    Serial.println(WiFi.localIP());
    blink();

    server.on("/", handleRoot);
    server.on("/SerialSend", serialSend);
    server.onNotFound(handleNotFound);
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

void bilnk_short(void) {
    digitalWrite(led, HIGH);
    delay(100);
    digitalWrite(led, LOW);
    delay(100);
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

void handleRoot(void) {
    server.send(200, "text/plain", "Hello world!");
    blink();
}

void handleNotFound(void) {
    server.send(404, "text/plain", "404: Not found");
    blink();
    blink();
}

void serialSend(void) {
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
    gabbySerial.write(first);
    wait_for(digitalRead(from_gabby) == LOW);
    wait_for(digitalRead(from_gabby) == HIGH);
    gabbySerial.write(second);
    wait_for(digitalRead(from_gabby) == LOW);
    wait_for(digitalRead(from_gabby) == HIGH);
    if ((first & 0xF0) == 0xA0) {
        wait_for(gabbySerial.available());
        byte buf[128];
        int i = 1;
        buf[0] = gabbySerial.read();
        if (buf[0] == 0xA4) {
            delayMicroseconds(2300);
            while (gabbySerial.available()) {
                buf[i] = gabbySerial.read();
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