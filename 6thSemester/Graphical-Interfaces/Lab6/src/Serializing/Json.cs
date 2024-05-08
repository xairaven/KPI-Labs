using System.IO;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace Lab6.Serializing;

public class Json
{
    public static void CustomSerialize<T>(string filePath, T data, bool referenceHandler = false)
    {
        var options = new JsonSerializerOptions
        {
            ReferenceHandler = referenceHandler? ReferenceHandler.Preserve : ReferenceHandler.IgnoreCycles,
            WriteIndented = true
        };
        var json = JsonSerializer.Serialize(data, options);
        File.WriteAllText(filePath, json);
    }
    
    public static T CustomDeserialize<T>(string filePath) where T : new()
    {
        if (!File.Exists(filePath))
        {
            return new T();
        }
        
        var json = File.ReadAllText(filePath);
        return JsonSerializer.Deserialize<T>(json)!;
    }

    public static string GetFilePath(string directory, string fileName)
    {
        return $"{directory}\\{fileName}.json";
    }
}