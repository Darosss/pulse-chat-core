using System.ComponentModel.DataAnnotations;

namespace message.Models;

public class RoomMember
{
    public int Id { get; set; }
    public int RoomId { get; set; }
    public Room? Room { get; set; }
    public int UserId { get; set; }

    [DataType(DataType.Date)]
    public DateTime JoinedAt { get; set; } = DateTime.UtcNow;
}
