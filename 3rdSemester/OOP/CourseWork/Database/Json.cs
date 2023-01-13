using System.Text.Json;
using System.Text.Json.Serialization;

namespace Shop.Database;

public static class Json
{
    private const string FilePath = "../../../Database/Data/";
    public static void CustomSerialize<T>(string fileName, T data, bool referenceHandler = false)
    {
        var options = new JsonSerializerOptions
        {
            ReferenceHandler = referenceHandler? ReferenceHandler.Preserve : ReferenceHandler.IgnoreCycles,
            WriteIndented = true
        };
        var json = JsonSerializer.Serialize(data, options);
        File.WriteAllText(FilePath + fileName + ".json", json);
    }
    
    public static T CustomDeserialize<T>(string fileName) where T : new()
    {
        string file = FilePath + fileName + ".json";
        if (!File.Exists(file))
        {
            return new T();
        }
        
        var json = File.ReadAllText(file);
        return JsonSerializer.Deserialize<T>(json)!;
    }
}