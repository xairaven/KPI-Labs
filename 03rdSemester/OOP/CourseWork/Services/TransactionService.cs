using System.Text.Json;
using Shop.Transactions;

namespace Shop.Services;

public static class TransactionService {
    public static List<Transaction> History { get; private set; }

    static TransactionService() {
        History = new List<Transaction>();
    }

    public static void Add(Transaction transaction)
    {
        History.Add(transaction);
    }

    public static List<Transaction> GetByAccountID(Guid accountId)
    {
        return History.Where(x => x.AccountId.Equals(accountId)).ToList();
    }

    public static void Deserialize(string fileName)
    {
        const string filePath = "../../../Database/Data/";
        
        string file = filePath + fileName + ".json";
        if (!File.Exists(file))
        {
            return;
        }

        var json = File.ReadAllText(file);
        History = JsonSerializer.Deserialize<List<Transaction>>(json)!;
    }
}