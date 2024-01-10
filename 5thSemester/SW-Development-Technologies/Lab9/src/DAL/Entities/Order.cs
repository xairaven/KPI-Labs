namespace DAL.Entities;

public class Order
{
    public int Id { get; set; }
    public int ClientId { get; set; }
    public string Title { get; set; }
    public string Origin { get; set; }
    public string Destination { get; set; }
    public int PeopleAmount { get; set; }
    public float Weight { get; set; }
    public float Height { get; set; }
}