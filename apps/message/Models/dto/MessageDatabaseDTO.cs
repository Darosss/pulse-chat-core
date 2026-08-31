namespace message.Models.Dto;

public sealed class MessageDatabaseDTO
{
    public required int ChannelId { get; set; }
    public required string Content { get; set; }
}
