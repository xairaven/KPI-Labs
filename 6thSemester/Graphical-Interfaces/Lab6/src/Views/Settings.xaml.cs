using System.Diagnostics;
using System.Windows;
using System.Windows.Navigation;
using Lab6.Context;
using Lab6.Enums;

namespace Lab6.Views;

public partial class Settings : Window
{
    private SettingsContext _context;
    
    public Settings(SettingsContext context)
    {
        InitializeComponent();

        _context = context;
    }

    private void CloseWindow(object sender, RoutedEventArgs e)
    {
        Close();
    }

    private void MusicBox_OnLoaded(object sender, RoutedEventArgs e)
    {
        MusicBox.ItemsSource = Enum.GetValues(typeof(Music)).Cast<Music>();

        var musicId = _context.Settings["MusicId"];
        MusicBox.SelectedItem = (Music)musicId;
        
        var duration = _context.Settings["AlarmDuration"];
        AlarmDurationSlider.Value = duration;

        var volume = _context.Settings["AlarmVolume"];
        AlarmVolumeSlider.Value = volume;
    }

    private void SaveSettings(object sender, RoutedEventArgs e)
    {
        var musicId = (int) MusicBox.SelectedItem;
        var alarmDuration = (int) AlarmDurationSlider.Value;
        var alarmVolume = (int)AlarmVolumeSlider.Value;

        _context.Settings["MusicId"] = musicId;
        _context.Settings["AlarmDuration"] = alarmDuration;
        _context.Settings["AlarmVolume"] = alarmVolume;
        
        _context.UpdateJson();
        
        Close();
    }

    private void Hyperlink_RequestNavigate(object sender, RequestNavigateEventArgs e)
    {
        Process.Start(new ProcessStartInfo(e.Uri.AbsoluteUri)
        {
            UseShellExecute = true
        });
        e.Handled = true;
    }
}