using Microsoft.EntityFrameworkCore;

namespace message.Data;

public class MessageDbContext(DbContextOptions<MessageDbContext> options) : DbContext(options)
{
    public DbSet<Models.Message> Messages { get; set; }
    public DbSet<Models.Room> Rooms { get; set; }
    public DbSet<Models.RoomMember> RoomMembers { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);

        modelBuilder.Entity<Models.RoomMember>().HasKey(rm => new { rm.RoomId, rm.UserId });

        modelBuilder
            .Entity<Models.RoomMember>()
            .HasOne(rm => rm.Room)
            .WithMany(r => r.Members)
            .HasForeignKey(rm => rm.RoomId)
            .OnDelete(DeleteBehavior.Cascade);
    }
}
