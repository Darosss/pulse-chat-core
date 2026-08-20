using Microsoft.EntityFrameworkCore;
namespace message.Data;

public class MessageDbContext(DbContextOptions<MessageDbContext> options): DbContext(options)

{
    public DbSet<Models.Message> Messages {get;set;}
}