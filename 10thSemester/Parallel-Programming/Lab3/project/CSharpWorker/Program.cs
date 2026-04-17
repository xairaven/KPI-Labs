using System.IO.MemoryMappedFiles;
using System.Net.Sockets;

namespace CSharpWorker;

class Program
{
    private static void Main(string[] args)
    {
        if (args.Length == 0) return;
        var mode = args[0];

        try
        {
            switch (mode)
            {
                case "pipe":
                    RunPipeMode();
                    break;
                case "tcp" when args.Length > 1:
                    RunTcpMode(int.Parse(args[1]));
                    break;
                case "shm" when args.Length > 1:
                    RunSharedMemoryMode(args[1]);
                    break;
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[C# Error] {ex.Message}");
        }
    }

    private static void RunPipeMode()
    {
        // Reading the number from standard input line by line
        var input = Console.ReadLine();
        if (!double.TryParse(input, out var number)) return;
        Console.Error.WriteLine($"[C# Worker] Received via Pipes: {number}");
        // Sending back via standard output
        Console.WriteLine(number);
    }

    private static void RunTcpMode(int port)
    {
        // Connecting to the local Rust server
        using var client = new TcpClient("127.0.0.1", port);
        using var stream = client.GetStream();

        var buffer = new byte[8];
        var bytesRead = stream.Read(buffer, 0, buffer.Length);

        if (bytesRead != 8) return;
        var number = BitConverter.ToDouble(buffer, 0);
        Console.Error.WriteLine($"[C# Worker] Received via TCP: {number}");
        stream.Write(buffer, 0, buffer.Length);
    }

    private static void RunSharedMemoryMode(string filePath)
    {
        // Opening the memory region created by Rust
        using var mmf =
            MemoryMappedFile.CreateFromFile(filePath, FileMode.Open, null, 8, MemoryMappedFileAccess.ReadWrite);
        using var accessor = mmf.CreateViewAccessor();

        var number = accessor.ReadDouble(0);
        Console.Error.WriteLine($"[C# Worker] Received via SHM: {number}");

        // Negating the number to flag that it was processed
        accessor.Write(0, -number);
    }
}