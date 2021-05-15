#include "secret.ino"
#include <ESP8266WiFi.h>

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

  pinMode(D3, OUTPUT);
}

void loop() {
    digitalWrite(D3, HIGH);
    delay(500);
    digitalWrite(D3, LOW);
    delay(500);

}