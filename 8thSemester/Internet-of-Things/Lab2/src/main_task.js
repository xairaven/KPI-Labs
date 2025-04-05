var smk_sensor = A0;
var livgFan = 2;
var frtDoor = 3;
var bWindow = 4;
var grgDoor = 5;

function setup() {
	pinMode(grgDoor, OUTPUT);
	pinMode(frtDoor, OUTPUT);
	pinMode(bWindow, OUTPUT);
	pinMode(livgFan, OUTPUT);
}

function loop() {
	// Read from sensor
	var newValue = analogRead(smk_sensor);
	newValue = newValue / 10;
	// Map it from 1023 to 256;
	// newValue = Math.floor(map(newValue, 0, 1023, 0, 256));
	
	if (newValue > 10) {
		customWrite(grgDoor, 1);
		customWrite(frtDoor, 1);
		customWrite(bWindow, 1);
		customWrite(livgFan, 2);
	} else {
		if (newValue < 1) {
			customWrite(grgDoor, 0);
			customWrite(frtDoor, 0);
			customWrite(bWindow, 0);
			customWrite(livgFan, 0);
		}
	}
	Serial.println("Smoke level: " + newValue);
}