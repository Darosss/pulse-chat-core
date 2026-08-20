using Grpc.Core;
using message.Data;
using Message;
namespace message.Services;

public class MessageServiceInternal(ChannelBroadcaster broadcaster, MessageDbContext dbContext, ILogger<MessageServiceInternal> logger) : MessageService.MessageServiceBase

{
    private readonly ChannelBroadcaster _broadcaster = broadcaster;
    private readonly MessageDbContext messageDb = dbContext;
    private readonly ILogger<MessageServiceInternal> _logger = logger;
  
    public override Task<HistoryResponse> GetChannelHistory(HistoryRequest request, ServerCallContext context)
    {
        var historyResponse = new HistoryResponse(){};
        historyResponse.Messages.Add([
            new()
        {
            Id=1,
            UserId=321,
            Content="Some test message",
            Timestamp=43141341343
        },
        new()
        {
            Id=12,
            UserId=3212,
            Content="2nd some test message",
            Timestamp=43141341345
        }
        ]);
        
        return Task.FromResult(historyResponse);

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
