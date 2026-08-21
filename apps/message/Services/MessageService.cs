using Grpc.Core;
using message.Data;
using Message;
using Microsoft.EntityFrameworkCore;
namespace message.Services;

public class MessageServiceInternal(ChannelBroadcaster broadcaster, MessageDbContext dbContext, ILogger<MessageServiceInternal> logger) : MessageService.MessageServiceBase

{
    private readonly ChannelBroadcaster _broadcaster = broadcaster;
    private readonly MessageDbContext messageDb = dbContext;
    private readonly ILogger<MessageServiceInternal> _logger = logger;
  
    public override async Task<HistoryResponse> GetChannelHistory(HistoryRequest request, ServerCallContext context)
    {
        var userId = context.RequestHeaders.GetValue("x-user-id");

        if (string.IsNullOrEmpty(userId) || !int.TryParse(userId, out int userIdAsNumber))
        {
            throw new RpcException(new Status(
                StatusCode.Unauthenticated, 
                "Missing proper x-user-id header in Gateway request"
            ));
        }

        var isMember = await this.messageDb.RoomMembers.AnyAsync(m=>m.RoomId == request.ChannelId && m.UserId == userIdAsNumber);
        if(!isMember)
        {
            throw new RpcException(new Status(
            StatusCode.PermissionDenied, 
            "You do not have access to this room"
        ));
        }
        var historyResponse = new HistoryResponse(){};


        var dbMessages = await this.messageDb.Messages.Where(m=>m.ChannelId==request.ChannelId).OrderByDescending(m=>m.Timestamp).Take((int)request.Limit).ToListAsync();

        
        var messageItems = dbMessages.Select((m)=>new MessageItem()
        {
            ChannelId=m.ChannelId,
            UserId=m.UserId,
            Content=m.Content,
            Timestamp=m.Timestamp.Ticks,
            Id=m.Id,
            
        });
        historyResponse.Messages.Add(messageItems);
        
        return historyResponse;

    }

    private async Task<Models.Message> SaveMessageToDatabase(CreateMessageRequest message) 
    {
        Models.Message dbMessage = new() {
            ChannelId=message.ChannelId,
            UserId=message.UserId,
            Content=message.Content, 
            Timestamp=DateTimeOffset.UtcNow.Date,
        };
        await this.messageDb.AddAsync(dbMessage);
        await this.messageDb.SaveChangesAsync();
        return dbMessage;
    }
    public override async Task<MessageItem> CreateMessage(CreateMessageRequest request, ServerCallContext context)
{
        var newMessage = await this.SaveMessageToDatabase(request);

        var messageItem = new MessageItem
        {
            Id = newMessage.Id,
            ChannelId=request.ChannelId,
            UserId = request.UserId,
            Content = request.Content,
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
        };

        await _broadcaster.BroadcastAsync(request.ChannelId, messageItem);
        return messageItem;
    }


    public override async Task StreamLiveMessages(StreamRequest request, IServerStreamWriter<MessageItem> responseStream, ServerCallContext context)
    {
        logger.LogInformation("Client connected to stream for channel: {ChannelId}", request.ChannelId);

        var reader = _broadcaster.Subscribe(request.ChannelId);

        try
        {
            while (!context.CancellationToken.IsCancellationRequested)
            {
                MessageItem newMessage = await reader.ReadAsync(context.CancellationToken);

                await responseStream.WriteAsync(newMessage);
                
                logger.LogInformation("Pushed message {MessageId} to stream", newMessage.Id);
            }
        }
        catch (OperationCanceledException)
        {
            logger.LogInformation("Client disconnected from stream: {ChannelId}", request.ChannelId);
        }
    }
}
