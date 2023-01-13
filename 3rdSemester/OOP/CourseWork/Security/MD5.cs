using System.Text;

namespace Shop.Security;

public static class MD5
{
    public static string Hash(string line)
    {
        byte[] lineInBytes = ASCIIEncoding.ASCII.GetBytes(line);
        byte[] hashInBytes = System.Security.Cryptography.MD5.HashData(lineInBytes);

        StringBuilder sb = new StringBuilder(hashInBytes.Length);
        foreach (var e in hashInBytes)
        {
            sb.Append(e.ToString("X2"));
        }

        return sb.ToString();
    }
}