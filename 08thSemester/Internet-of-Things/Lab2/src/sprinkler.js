var sensor = 0;
var sprinkler = 1;

var MIN_ACCEPTABLE_LEVEL_CM = 3;
var MAX_ACCEPTABLE_LEVEL_CM = 12;

function setup() {
	pinMode(sensor, INPUT);
  	pinMode(sprinkler, OUTPUT);
}

function loop() {
  	var water_level_cm = analogRead(sensor);
  	water_level_cm = Math.floor(map(water_level_cm, 0, 1023, 0, 20));
  
    if (water_level_cm < MIN_ACCEPTABLE_LEVEL_CM) {
      customWrite(sprinkler, 1);
    }
    if (water_level_cm > MAX_ACCEPTABLE_LEVEL_CM) {
      customWrite(sprinkler, 0);
    }
    
    Serial.println("Water Level (CM): " + water_level_cm);
}