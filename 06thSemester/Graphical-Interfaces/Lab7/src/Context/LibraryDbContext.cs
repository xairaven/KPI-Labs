using System.Configuration;
using Lab7.Entities;
using Microsoft.EntityFrameworkCore;

namespace Lab7.Context;

public partial class LibraryDbContext : DbContext
{
    public LibraryDbContext()
    {
    }

    public LibraryDbContext(DbContextOptions<LibraryDbContext> options)
        : base(options)
    {
    }

    public virtual DbSet<Book> Books { get; set; }

    public virtual DbSet<Publisher> Publishers { get; set; }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
        => optionsBuilder.UseSqlServer(ConfigurationManager.
            ConnectionStrings["LibraryDatabase"].ConnectionString);

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<Book>(entity =>
        {
            entity.HasKey(e => e.Isbn).HasName("PK__Books__99F9D0A5327E0CAC");

            entity.Property(e => e.Isbn)
                .HasMaxLength(30)
                .IsUnicode(false)
                .HasColumnName("isbn");
            entity.Property(e => e.Authors)
                .HasMaxLength(255)
                .IsUnicode(false)
                .HasColumnName("authors");
            entity.Property(e => e.PublicationYear).HasColumnName("publication_year");
            entity.Property(e => e.PublisherCode).HasColumnName("publisher_code");
            entity.Property(e => e.Title)
                .HasMaxLength(255)
                .IsUnicode(false)
                .HasColumnName("title");

            entity.HasOne(d => d.PublisherCodeNavigation).WithMany(p => p.Books)
                .HasForeignKey(d => d.PublisherCode)
                .OnDelete(DeleteBehavior.ClientSetNull)
                .HasConstraintName("FK_PublishersCode");
        });

        modelBuilder.Entity<Publisher>(entity =>
        {
            entity.HasKey(e => e.Id).HasName("PK__Publishe__3213E83F979AC652");

            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.Name)
                .HasMaxLength(255)
                .IsUnicode(false)
                .HasColumnName("name");
        });

        OnModelCreatingPartial(modelBuilder);
    }

    partial void OnModelCreatingPartial(ModelBuilder modelBuilder);
}
