using System.ComponentModel.DataAnnotations;

namespace message.Models;

public class Message
{
    public int Id {get; set;}
    public required string UserId {get;set;}

    [DataType(DataType.Date)]
    public required DateTime Timestamp {get;set;}
    public required string Content {get;set;}

    
}