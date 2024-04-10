using System.Configuration;
using Lab4.Database;

namespace Lab4;

/// <summary>
/// Interaction logic for App.xaml
/// </summary>
public partial class App
{
    public App()
    {
        var connectionString = ConfigurationManager.ConnectionStrings["connectionString_ADO"]
            .ConnectionString;
        
        AdoWrapper.ConfigureConnectionString(connectionString);
    }
}