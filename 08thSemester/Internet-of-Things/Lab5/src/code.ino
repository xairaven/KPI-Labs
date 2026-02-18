#include <WiFi.h>
#include <WiFiClientSecure.h>  // Secure TLS client
#include <PubSubClient.h>
#include <ArduinoJson.h>

// Wi-Fi Parameters
const char* WIFI_SSID = "Wokwi-GUEST";
const char* WIFI_PASSWORD = "";

// MQTT Server Parameters (TLS)
const char* MQTT_CLIENT_ID = "ESPLab5";
const char* MQTT_BROKER = "9635d68c61b74e0a9e7adb108b924247.s1.eu.hivemq.cloud";
const char* MQTT_USER = "SlaveClient";
const char* MQTT_PASSWORD = "RootP@ss1";
const char* MQTT_TOPIC = "Lab5_Temperature_Topic";
const int MQTT_PORT = 8883;  // Secure MQTT Port

WiFiClientSecure espClient;
PubSubClient client(espClient);

const int photoresistorPin = 34;
const int triggerPin = 4;
const int echoPin = 2;
const int lightWarningBubblePin = 5;
bool turned = true;
bool alert = false;

// Callback function for incoming MQTT messages
void callback(char* topic, byte* payload, unsigned int length) {
  Serial.print("Message received on topic: ");
  Serial.println(topic);

  String message;
  for (int i = 0; i < length; i++) {
    message += (char)payload[i];
  }
  Serial.print("Payload: ");
  Serial.println(message);

  DynamicJsonDocument doc(256);
  DeserializationError error = deserializeJson(doc, message);

  if (error) {
    Serial.print(F("Failed to parse JSON: "));
    Serial.println(error.c_str());
    return;
  }

  String stringtext = doc["Status"];
  if (stringtext == "off") {
    turned = false;
  } else if (stringtext == "on") {
    turned = true;
  }
}

void setup() {
  Serial.begin(115200);
  pinMode(photoresistorPin, INPUT);
  pinMode(triggerPin, OUTPUT);
  pinMode(echoPin, INPUT);
  pinMode(lightWarningBubblePin, OUTPUT);

  // Connect to WiFi
  Serial.print("Connecting to WiFi");
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
  while (WiFi.status() != WL_CONNECTED) {
    Serial.print(".");
    delay(100);
  }
  Serial.println(" Connected!");

  // Secure connection for TLS
  espClient.setInsecure();  // Skips certificate validation (use only if needed)

  client.setServer(MQTT_BROKER, MQTT_PORT);
  client.setCallback(callback);  // Set the callback function for incoming messages
  client.setKeepAlive(60);

  Serial.print("Connecting to MQTT server... ");
  while (!client.connected()) {
    if (client.connect(MQTT_CLIENT_ID, MQTT_USER, MQTT_PASSWORD)) {
      Serial.println("Connected!");
      client.subscribe(MQTT_TOPIC);  // Subscribe to MQTT topic
    } else {
      Serial.print("Failed, rc=");
      Serial.print(client.state());
      Serial.println(" Retrying in 5 seconds...");
      delay(5000);
    }
  }
}

void loop() {
  client.loop();
  String message = "";

  if (analogRead(photoresistorPin) > 2000 && turned) {
    digitalWrite(lightWarningBubblePin, HIGH);
    
    // Trigger ultrasonic sensor
    digitalWrite(triggerPin, LOW);
    delay(2);
    digitalWrite(triggerPin, HIGH);
    delay(10);
    digitalWrite(triggerPin, LOW);
    
    long value = pulseIn(echoPin, HIGH);
    double meters = value / 58.0;  // Convert to centimeters

    Serial.print("Distance: ");
    Serial.print(meters);
    Serial.println(" cm");

    if (meters > 200) {
      digitalWrite(lightWarningBubblePin, LOW);
      if (alert) {
        alert = false;
        message = "{\"Alert\":\"All Good!\"}";
        client.publish(MQTT_TOPIC, message.c_str(), true);
        Serial.print("Broker Status:");
        Serial.print(client.state());
        Serial.println();
      }
    } else {
      alert = true;
      message = "{\"Alert\":\"Someone Close!\"}";
      client.publish(MQTT_TOPIC, message.c_str(), true);
      Serial.print("Broker Status:");
      Serial.print(client.state());
      Serial.println();
    }
  } else if (analogRead(photoresistorPin) > 2000) {
    message = "{\"Alert\":\"System disabled!\"}";
    client.publish(MQTT_TOPIC, message.c_str(), true);
    Serial.print("Broker Status:");
    Serial.print(client.state());
    Serial.println();
  }

  client.loop();
  delay(5000);
}
