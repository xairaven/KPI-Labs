import arcpy
import os
import sys

# Workspace settings
arcpy.env.workspace = os.getcwd()
arcpy.env.overwriteOutput = True

# Args input
Input_File = sys.argv[1]
Distance = sys.argv[2]
Output_File = sys.argv[3]

# Arguments by default
Line_Side = "FULL"
Line_End_Type = "ROUND"
Dissolve_Type = "ALL"
Dissolve_Option = ""
Method = "PLANAR"

# Process: Buffer
arcpy.Buffer_analysis(Input_File, Output_File, Distance, Line_Side, Line_End_Type, Dissolve_Type, Dissolve_Option,
                      Method)