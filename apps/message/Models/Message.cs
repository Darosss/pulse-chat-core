using System.ComponentModel.DataAnnotations;

namespace message.Models;

public class Message
{
    public int Id { get; set; }
    public required int UserId { get; set; }
    public required int ChannelId { get; set; }

    [DataType(DataType.Date)]
    public required DateTime Timestamp { get; set; }
    public required string Content { get; set; }
}
