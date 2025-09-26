#include <WiFi.h>
#include <HTTPClient.h>
#include <TFT_eSPI.h>

#include "secrets.h"

TFT_eSPI tft = TFT_eSPI();

float parseNum(const String&s,const char*key){
  int k=s.indexOf(key); if(k<0) return NAN;
  int c=s.indexOf(':',k); if(c<0) return NAN;
  int i=c+1; while(i<s.length()&&isSpace(s[i])) i++;
  int j=i; while(j<s.length()&&((s[j]>='0'&&s[j]<='9')||s[j]=='-'||s[j]=='.')) j++;
  return s.substring(i,j).toFloat();
}

void setup(){
  // initialize display
  tft.init(); tft.setRotation(1); tft.setSwapBytes(true); tft.invertDisplay(true);
  tft.fillScreen(0x0000); tft.setTextSize(2); tft.setTextColor(0xFFFF,0x0000); tft.setTextDatum(TL_DATUM);

  // connect to wifi
  tft.drawString("Connecting to WiFi...",0,0);
  WiFi.begin(ssid,password); while(WiFi.status()!=WL_CONNECTED) delay(500);

}

void loop(){
  tft.drawString("Fetching...",5,224);
  unsigned long startTime = millis();

  HTTPClient http;
  http.begin(url);
  int code = http.GET();
  if(code == 200){
    String body = http.getString();
    int idx = body.indexOf("\"current_weather\":");
    if(idx >= 0){
      String cw = body.substring(idx);
      tft.fillScreen(0x0000);
      float temp = parseNum(cw,"\"temperature\"");
      float wind = parseNum(cw,"\"windspeed\"");


      String title = " -- Brighton Weather --";
      tft.drawString(title, (tft.width() - tft.textWidth(title)) / 2, 4);

      String temperature = "Temperature: " + String(temp, 1) + " C";
      tft.drawString(temperature, (tft.width() - tft.textWidth(temperature)) / 2, 36);

      String windSpeed = "Wind Speed: " + String(wind,1) + " km/h";
      tft.drawString(windSpeed, (tft.width() - tft.textWidth(windSpeed)) / 2, 60);

      int tempInt = (int)temp;
      String tempMessage;

      switch (tempInt) {
        case 0 ... 8:
          tempMessage = "Freezing!!";
          break;
        case 9 ... 14:
          tempMessage = "Pretty cold!";
          break;
        case 15 ... 22:
          tempMessage = "Just right!";
          break;
        case 23 ... 30:
          tempMessage = "It's hot!";
          break;
        default:
          tempMessage = "Extreme weather!";
          break;
      }

      tft.drawString(tempMessage, (tft.width() - tft.textWidth(tempMessage)) / 2, 180);
    }
  }
  http.end();
  delay(5000);
}
