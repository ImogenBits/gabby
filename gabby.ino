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
const byte from_gabby = D4;


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
    pinMode(D3, OUTPUT);
    digitalWrite(D3, LOW);

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
    digitalWrite(D3, HIGH);
    delay(500);
    digitalWrite(D3, LOW);
    delay(500);
}

void bilnk_short(void) {
    digitalWrite(D3, HIGH);
    delay(100);
    digitalWrite(D3, LOW);
    delay(100);
}

String bytes_to_hex(byte *buf, int count) {
    String out = String();
    char bleh[3];
    for (int i = 0; i < count; i++) {
        sprintf(bleh, "%x", buf[i]);
        out += bleh;
    }
    return bleh;
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
    if (! server.hasArg("data") || server.arg("data") == NULL) {
        server.send(400, "text/plain", "invalid re");
        return;
    }
    Serial.println(server.arg("data"));

    if (server.arg("data") == "Online") {
        switchOnline();
    } else if (server.arg("data") == "Offline") {
        switchOffline();
    } else {
        sendCommand(server.arg("data"));
    }

    blink();
    server.send(200, "text/plain", "Sent data");
}

void switchOnline(void) {
    byte data[] = {0xA0, 0x00, 0xA1, 0x00, 0xA4, 0x00, 0xA2, 0x00};
    sendBytes(data, 8);
}

void switchOffline(void) {
    byte data[] = {0xA3, 0x00, 0xA0, 0x00};
    sendBytes(data, 4);
}

void sendBytes(byte data[], byte count) {
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
    wait_for(gabbySerial.available());
    byte response = gabbySerial.read();
    String out = String();
    if (response == 0xA4) {
        byte buf[128];
        int i = 0;
        delay(2);
        while (gabbySerial.available()) {
            buf[i++] = gabbySerial.read();
            delay(2);
        }
        out = bytes_to_hex(buf, i);
    }
    digitalWrite(to_gabby, LOW);
    delay(1);
    digitalWrite(to_gabby, HIGH);
    return out;
}

void sendCommand(const String &cmd) {
    char c[5];
    cmd.toCharArray(c, 5);
    int a = strtol(c, NULL, 16);
    Serial.printf("%d\n", a);
    byte data[] = {a >> 8, (byte) a};
}