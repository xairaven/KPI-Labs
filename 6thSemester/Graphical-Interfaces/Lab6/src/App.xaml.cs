using System.Configuration;
using System.IO;
using System.Windows;

namespace Lab6;

/// <summary>
/// Interaction logic for App.xaml
/// </summary>
public partial class App : Application
{
    public App()
    {
        // Register Syncfusion license
        var key = ConfigurationManager.AppSettings["SyncFusionKey"];
        Syncfusion.Licensing.SyncfusionLicenseProvider.RegisterLicense(key);
        
        // Setting DataDirectory
        var executable = System.Reflection.Assembly.GetExecutingAssembly().Location;
        var path = Path.GetDirectoryName(executable);
        AppDomain.CurrentDomain.SetData("DataDirectory", path);
    }
}