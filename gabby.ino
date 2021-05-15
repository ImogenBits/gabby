#include "secret.h"
#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>

#define SERVER_PORT 80

ESP8266WebServer server(80);

void handleRoot() {
    blink();
    server.send(200, "text/plain", "Hello world!");
    blink();
}

void handleNotFound() {
    blink();
    blink();
    server.send(404, "text/plain", "404: Not found");
    blink();
    blink();
}


void blink(void) {
    digitalWrite(D3, HIGH);
    delay(500);
    digitalWrite(D3, LOW);
    delay(500);
}


void setup()
{
    Serial.begin(115200);
    Serial.println();

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
    server.on("/SerialSend", )
    server.onNotFound(handleNotFound);
    server.begin();

    pinMode(D3, OUTPUT);
    digitalWrite(D3, LOW);
}

void loop() {
    server.handleClient();
}