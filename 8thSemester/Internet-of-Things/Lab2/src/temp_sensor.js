var sensor = A0;
var furnace = 2;

var MIN_ACCEPTABLE_TEMP = 20;
var MAX_ACCEPTABLE_TEMP = 27;

function setup() {
  pinMode(furnace, OUTPUT);
}

function loop() {
	var temperature = analogRead(sensor);
	temperature = Math.floor(map(temperature, 0, 1023, -100, 100));
  
  	if (temperature < MIN_ACCEPTABLE_TEMP) {
  		digitalWrite(furnace, HIGH);
  	}
  	if (temperature > MAX_ACCEPTABLE_TEMP) {
  		digitalWrite(furnace, LOW);
  	}
  	
  	Serial.println("Temperature (Celcius): " + temperature);
}