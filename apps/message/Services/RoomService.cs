using Grpc.Core;
using message.Data;
using message.Models;
using Microsoft.EntityFrameworkCore;

namespace message.Services;

public class RoomService(MessageDbContext dbContext, ILogger<RoomService> logger)
{
    public async Task<bool> IsUserIdAMemberOfRoom(int userId, int channelId)
    {
        return await dbContext.RoomMembers.AnyAsync(m =>
            m.RoomId == channelId && m.UserId == userId
        );
    }

    public async Task<int> GetOrCreateDirectRoomAsync(int user1Id, int user2Id)
    {
        if (user1Id == user2Id)
        {
            throw new RpcException(
                new Status(StatusCode.InvalidArgument, "Cannot get a room with yourself")
            );
        }
        var existingRoomId = await dbContext
            .Rooms.Where(r => r.Type == RoomType.Direct)
            .Where(r =>
                r.Members.Any(m => m.UserId == user1Id) && r.Members.Any(m => m.UserId == user2Id)
            )
            .Select(r => r.Id)
            .FirstOrDefaultAsync();

        if (existingRoomId != default)
        {
            return existingRoomId;
        }

        var newRoom = new Room
        {
            Type = RoomType.Direct,
            Members = [new() { UserId = user1Id }, new() { UserId = user2Id }],
        };

        dbContext.Rooms.Add(newRoom);

        await dbContext.SaveChangesAsync();

        return newRoom.Id;
    }
}
