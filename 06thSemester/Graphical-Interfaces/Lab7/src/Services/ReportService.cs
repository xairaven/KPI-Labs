using System.IO;
using System.Text;
using System.Windows.Controls;
using System.Windows.Forms;

namespace Lab7.Services;

public class ReportService
{
    public string Filename { get; set; }
    
    public ReportService(string filename)
    {
        Filename = filename;
    }
    
    public string ToCsvString(DataGrid dataGrid)
    {
        if (dataGrid is null) throw new ArgumentNullException(nameof(dataGrid));

        // Retrieve the items source of the DataGrid
        if (dataGrid.ItemsSource is not IEnumerable<object> itemsSource)
            throw new InvalidOperationException("DataGrid has no items source");

        // Get the headers from the columns
        var headers = dataGrid.Columns.Select(c => c.Header.ToString()).ToList();

        // Create a list of CSV lines
        var csvLines = new List<string>();

        // Add the headers line
        csvLines.Add(string.Join(",", headers));

        // Iterate through each item in the DataGrid
        foreach (var item in itemsSource)
        {
            // Create a list of values for each column
            List<string> values = new List<string>();
            foreach (var column in dataGrid.Columns)
            {
                var binding = column.SortMemberPath ?? column.SortMemberPath;

                // Get the property value of the current item using reflection
                var propertyInfo = item.GetType().GetProperty(binding!);
                var value = propertyInfo?.GetValue(item)?.ToString() ?? "";

                // Add the value to the list of values
                values.Add(value);
            }

            // Join the values with commas and add to csv lines
            csvLines.Add(string.Join(",", values));
        }

        var sb = new StringBuilder();
        foreach (var line in csvLines)
        {
            sb.Append($"{line}\n");
        }

        return sb.ToString();
    }

    public void ToTextFile(string text)
    {
        var path = Path.GetDirectoryName(Application.ExecutablePath);
        var finalPath = $"{path}/{Filename}";
        
        File.WriteAllText(finalPath, text);
    }
}