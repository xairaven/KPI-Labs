using System;
using System.Collections.Generic;

namespace Lab5.Entities;

public partial class Book
{
    public string Isbn { get; set; } = null!;

    public string Title { get; set; } = null!;

    public string Authors { get; set; } = null!;

    public int PublisherCode { get; set; }

    public short? PublicationYear { get; set; }

    public virtual Publisher PublisherCodeNavigation { get; set; } = null!;
}
