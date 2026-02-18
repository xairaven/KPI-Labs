import arcpy
import os

# Workspace settings
arcpy.env.workspace = os.getcwd()
arcpy.env.overwriteOutput = True

# Greetings!
print("Hello! This script makes buffer from your shape file.")
print("Author: Alex Kovalov, TP-12\n")

# Args input
Input_File = input("Input SHP filename: ")
Distance = input("Input distance: ")
Output_File = input("Result SHP filename: ")

# Arguments by default
Line_Side = "FULL"
Line_End_Type = "ROUND"
Dissolve_Type = "ALL"
Dissolve_Option = ""
Method = "PLANAR"

# Process: Buffer
arcpy.Buffer_analysis(Input_File, Output_File, Distance, Line_Side, Line_End_Type, Dissolve_Type, Dissolve_Option,
                      Method)