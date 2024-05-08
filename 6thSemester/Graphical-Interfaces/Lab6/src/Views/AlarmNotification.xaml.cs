using System.IO;
using System.Windows;
using System.Windows.Media;
using Lab6.Context;
using Lab6.Entities;
using Lab6.Enums;

namespace Lab6.Views;

public partial class AlarmNotification : Window
{
    private SettingsContext _context;
    private MediaPlayer _mediaPlayer;
    private Alarm _record;
    
    public AlarmNotification(SettingsContext context, Alarm record)
    {
        _context = context;
        _record = record;
        
        _mediaPlayer = new MediaPlayer();
        var volume = (double)_context.Settings["AlarmVolume"] / 10;
        _mediaPlayer.Volume = volume;
        
        Closed += OnClosed;
        
        OpenMusicFile(_context.Settings["MusicId"]);
        
        InitializeComponent();
        InitializeFields();
        
        StopMusicButton.IsEnabledChanged += StopMusicButtonOnIsEnabledChanged;
        
        NotificationMusicStart(_context.Settings["AlarmDuration"]);
    }

    private void StopMusicButtonOnIsEnabledChanged(object sender, DependencyPropertyChangedEventArgs e)
    {
        StopMusicBorder.Background = new SolidColorBrush(Colors.DarkGray);
    }

    private void InitializeFields()
    {
        if (!_record.Title.Equals(""))
        {
            TitleLabel.Content = _record.Title;
        } else TitleLabel.Content = "None";
        
        var dateTime = _record.Datetime;
        TimeLabel.Content = $"{dateTime.Hour:00}:{dateTime.Minute:00}";
        DateLabel.Content = $"{dateTime.Day:00}.{dateTime.Month:00}.{dateTime.Year:0000}";
    }

    private void OnClosed(object? sender, EventArgs e)
    {
        _mediaPlayer.Stop();
    }


    private void OpenMusicFile(int musicId)
    {
        var filePath = $"Assets\\Music\\{((Music)musicId).ToString()}.mp3";

        var uri = new Uri(filePath, UriKind.Relative);

        if (!File.Exists(Path.Combine(Path.GetDirectoryName(AppDomain.CurrentDomain.BaseDirectory)!, uri.ToString())))
        {
            MessageBox.Show(messageBoxText:"Something wrong with music file uploading.", 
                caption: "Error!", 
                button: MessageBoxButton.OK, 
                icon: MessageBoxImage.Error, 
                defaultResult: MessageBoxResult.OK);
            
            Application.Current.Shutdown();
        }
        
        _mediaPlayer.Open(uri);
    }

    private async void NotificationMusicStart(int duration)
    {
        PlayMusic(null, null);
        _mediaPlayer.MediaEnded += PlayMusic;

        await Task.Run(() => Thread.Sleep(TimeSpan.FromSeconds(duration)));
        if (_mediaPlayer.CanPause)
        {
            _mediaPlayer.Stop();
            StopMusicButton.IsEnabled = false;
        }
    }
    
    private void PlayMusic(object? sender, EventArgs? e)
    {
        _mediaPlayer.Position = TimeSpan.Zero;
        _mediaPlayer.Play();
    }

    private void StopMusic_OnClick(object sender, RoutedEventArgs e)
    {
        StopMusicButton.IsEnabled = false;

        if (_mediaPlayer.CanPause)
        {
            _mediaPlayer.Stop();
        }
    }
}