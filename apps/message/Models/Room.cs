using System.ComponentModel.DataAnnotations;

namespace message.Models;

public enum RoomType
{
    GuildText = 0,
    Direct = 1,
    Group = 2,
}

public class Room
{
    public int Id { get; set; }

    public RoomType Type { get; set; } = RoomType.Direct;

    public string? Name { get; set; }

    public int? GuildId { get; set; }

    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;

    public ICollection<RoomMember> Members { get; set; } = new List<RoomMember>();
    public ICollection<Message> Messages { get; set; } = new List<Message>();
}
