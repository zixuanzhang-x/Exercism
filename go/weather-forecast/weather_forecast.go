// Package weather does ...
package weather

// CurrentCondition does ...
var CurrentCondition string 

// CurrentLocation does ...
var CurrentLocation string 

// Forecast does ...
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
