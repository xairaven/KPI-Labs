using System.Windows.Input;

namespace Lab7.Commands;

public static class DataCommands
{
    public static RoutedCommand Undo { get; }
    public static RoutedCommand Create { get; }
    public static RoutedCommand Edit { get; }
    public static RoutedCommand Save { get; }
    public static RoutedCommand Find { get; }
    public static RoutedCommand Reload { get; }
    public static RoutedCommand Delete { get; }
    public static RoutedCommand Report { get; }
    
    static DataCommands() {
        Undo = new RoutedCommand("Delete", typeof(DataCommands),
            [new KeyGesture(Key.Z, ModifierKeys.Control, "Ctrl+Z")]);
        
        Create = new RoutedCommand("Create", typeof(DataCommands),
            [new KeyGesture(Key.N, ModifierKeys.Control, "Ctrl+N")]);
        
        Edit = new RoutedCommand("Edit", typeof(DataCommands),
            [new KeyGesture(Key.E, ModifierKeys.Control, "Ctrl+E")]);
        
        Save = new RoutedCommand("Save", typeof(DataCommands),
            [new KeyGesture(Key.S, ModifierKeys.Control, "Ctrl+S")]);
        
        Find = new RoutedCommand("Find", typeof(DataCommands),
            [new KeyGesture(Key.F, ModifierKeys.Control, "Ctrl+F")]);
        
        Reload = new RoutedCommand("Reload", typeof(DataCommands),
            [new KeyGesture(Key.R, ModifierKeys.Control, "Ctrl+R")]);
        
        Delete = new RoutedCommand("Delete", typeof(DataCommands),
            [new KeyGesture(Key.D, ModifierKeys.Control, "Ctrl+D")]);
        
        Report = new RoutedCommand("Report", typeof(DataCommands),
            [new KeyGesture(Key.P, ModifierKeys.Control, "Ctrl+P")]);
    }
}