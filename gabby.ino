#include "secret.h"
#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SoftwareSerial.h>

#define SERVER_PORT 80
const byte txPin = D7;
const byte rxPin = D5;
#define DELAY 1


ESP8266WebServer server(80);

SoftwareSerial typeSerial(rxPin, txPin, true);

void blink(void);

void handleRoot(void);
void handleNotFound(void);
void serialSend(void);

void switchOnline(void);
void switchOffline(void);
void sendCommand(const String &cmd);

void setup() {
    pinMode(rxPin, INPUT);
    pinMode(txPin, OUTPUT);
    pinMode(D3, OUTPUT);
    digitalWrite(D3, LOW);
    pinMode(D6, OUTPUT);
    digitalWrite(D6, LOW);

    Serial.begin(115200);
    Serial.println();

    typeSerial.begin(4800);

    WiFi.begin(WIFI_NAME, WIFI_PW);
    Serial.print("Connecting");
    while (WiFi.status() != WL_CONNECTED)
    {
    delay(500);
    Serial.print(".");
    }
    Serial.println();
    Serial.print("Connected, IP address: ");
    Serial.println(WiFi.localIP());

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
    digitalWrite(D6, LOW);
    delay(DELAY);
    typeSerial.write(0xA0);
    delay(DELAY);
    typeSerial.write(0x00);
    delay(DELAY);
    typeSerial.write(0xA1);
    delay(DELAY);
    typeSerial.write(0x00);
    delay(DELAY);
    typeSerial.write(0xA4);
    delay(DELAY);
    typeSerial.write(0x00);
    delay(DELAY);
    typeSerial.write(0xA2);
    delay(DELAY);
    typeSerial.write(0x00);
    delay(DELAY);
    digitalWrite(D6, HIGH);
}

void switchOffline(void) {
    delay(DELAY);
    typeSerial.write(0xA3);
    delay(DELAY);
    typeSerial.write(0x00);
    delay(DELAY);
    typeSerial.write(0xA0);
    delay(DELAY);
    typeSerial.write(0x00);
    delay(DELAY);
}

void sendBytes(byte fst, byte snd) {
    digitalWrite(D6, LOW);
    delay(5);
    delay(DELAY);
    delay(DELAY);
    typeSerial.write(fst);
    delay(DELAY);
    typeSerial.write(snd);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    delay(DELAY);
    digitalWrite(D6, HIGH);
}

void sendCommand(const String &cmd) {
    char c[5];
    cmd.toCharArray(c, 5);
    int a = strtol(c, NULL, 16);
    Serial.printf("%d\n", a);
    sendBytes(a >> 8, (byte) a);
}