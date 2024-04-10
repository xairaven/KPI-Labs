using System.Configuration;
using System.Data;
using System.Data.SqlClient;
using System.Windows;

namespace Lab4.Database;

public class AdoWrapper
{
    private static string? _connectionString;
    
    public AdoWrapper()
    {
        try
        {
            if (_connectionString is null)
                throw new SettingsPropertyNotFoundException("Connection string is uninitialized");

            var connection = new SqlConnection(connectionString: _connectionString);
            
            connection.Open();
        }
        catch (SqlException)
        {
            MessageBox.Show(messageBoxText: "No connection with SQL Database.",
                caption: "Error!",
                button: MessageBoxButton.OK,
                icon: MessageBoxImage.Error,
                defaultResult: MessageBoxResult.OK);
            
            Application.Current.Shutdown();
        }
    }
    
    public static void ConfigureConnectionString(string connectionString)
    {
        _connectionString = connectionString;
    }

    public async Task<int> ExecuteNonQuery(string query)
    {
        int rowsAffected;
        
        await using (var connection = new SqlConnection(_connectionString))
        {
            await connection.OpenAsync();

            var command = new SqlCommand(cmdText: query, 
                connection: connection);
            
            rowsAffected = await command.ExecuteNonQueryAsync();
        }

        return rowsAffected;
    }
    
    public async Task<DataTable> ExecuteReader(string query)
    {
        await using var connection = new SqlConnection(_connectionString);
        await connection.OpenAsync();

        var adapter = new SqlDataAdapter(query, connection);
        var table = new DataTable();
        adapter.Fill(table);
        
        return table;
    }
    
    public async Task<double> ExecuteScalar(string query)
    {
        await using var connection = new SqlConnection(_connectionString);
        await connection.OpenAsync();

        var command = new SqlCommand(cmdText: query, 
            connection: connection);
            
        var value = (double) (await command.ExecuteScalarAsync() ?? throw new InvalidOperationException());

        return value;
    }
}