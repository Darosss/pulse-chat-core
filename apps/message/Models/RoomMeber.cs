using System.ComponentModel.DataAnnotations;

namespace message.Models;

public class RoomMember
{
    public int Id {get; set;}
    public required int RoomId {get;set;}
    public required int UserId {get;set;}
    [DataType(DataType.Date)]
    public required DateTime JoinedAt {get;set;}

    
}