#!/bin/bash

# Welcome the user
echo "Welcome, $USER!"

# Display current date and time
current_datetime=$(date "+%Y-%m-%d %H:%M:%S")
echo "Current date and time: $current_datetime"

# Determine the time of day
current_hour=$(date "+%H")
if (( current_hour >= 0 && current_hour < 6 )); then
    time_of_day="night"
elif (( current_hour >= 6 && current_hour < 12 )); then
    time_of_day="morning"
elif (( current_hour >= 12 && current_hour < 18 )); then
    time_of_day="day"
else
    time_of_day="evening"
fi
echo "Time of day: $time_of_day"

# Find the line in the /etc/passwd file using grep
# and output the seven fields of that line using cut
echo "User information:"
grep $USER /etc/passwd | cut -d ":" -f 1- --output-delimiter $'\n'