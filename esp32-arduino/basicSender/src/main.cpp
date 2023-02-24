#include <Arduino.h>
#include <WiFi.h>
#include <esp_now.h>

uint8_t broadcastAddress[] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff};
/* uint8_t broadcastAddress[] = {0xa2, 0x44, 0x4a, 0xe4, 0xb6, 0x74}; */




typedef struct struct_message
{
  char a[32];
  int b;
  float c;
  bool d;
} struct_message;

struct_message myData;

esp_now_peer_info_t peerInfo;

// callback when data is sent
void OnDataSent(const uint8_t *mac_addr, esp_now_send_status_t status)
{
  Serial.print("\r\nLast Packet Send Status:\t");
  Serial.println(status == ESP_NOW_SEND_SUCCESS ? "Delivery Success" : "Delivery Fail");
}

void setup()
{
  Serial.begin(115200);
  WiFi.mode(WIFI_MODE_STA);
  Serial.println(WiFi.macAddress());

  if (esp_now_init() != ESP_OK)
  {
    Serial.println("Error initializing ESP-NOW");
    return;
  }

  esp_now_register_send_cb(OnDataSent);
  memcpy(peerInfo.peer_addr, broadcastAddress, 6);
  peerInfo.channel = 1;
  peerInfo.encrypt = false;

  // Add peer
  if (esp_now_add_peer(&peerInfo) != ESP_OK){
    Serial.println("Failed to add peer");
    return;
  }
}

void loop()
{
  // Set values to send
  strcpy(myData.a, "THIS IS A CHAR");
  myData.b = random(1, 20);
  myData.c = 1.2;
  myData.d = false;

  // Send message via ESP-NOW
  esp_err_t result = esp_now_send(broadcastAddress, (uint8_t *)&myData, sizeof(myData));

  if (result == ESP_OK)
  {
    Serial.println("Sent with success");
  }
  else
  {
    Serial.println("Error sending the data");
  }
  delay(2000);
}
