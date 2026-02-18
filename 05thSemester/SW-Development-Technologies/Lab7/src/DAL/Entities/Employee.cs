using DAL.Enums;

namespace DAL.Entities;

public class Employee
{
    public required int Id { get; set; }
    public required string FirstName { get; set; }
    public string? Patronymic { get; set; }
    public required string LastName { get; set; }
    public string? Email { get; set; }
    public string? Phone { get; set; }
    public required IEnumerable<Role> Roles { get; set; }
    public required string Password { get; set; }
    public required Guid Salt { get; set; }
}