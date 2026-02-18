namespace DAL.Entities;

public class Order
{
    public required int Id { get; set; }
    public required int ClientId { get; set; }
    public required string Title { get; set; }
    public required string Origin { get; set; }
    public required string Destination { get; set; }
    public int PeopleAmount { get; set; }
    public float Weight { get; set; }
    public float Height { get; set; }
}