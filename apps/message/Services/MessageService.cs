using Grpc.Core;
using message.Data;
using Message;
using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore.Metadata.Internal;

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
            MessageId="1",
            UserId="321",
            Content="Some test message",
            Timestamp=43141341343
        },
        new()
        {
            MessageId="12",
            UserId="3212",
            Content="2nd some test message",
            Timestamp=43141341345
        }
        ]);
        
        return Task.FromResult(historyResponse);

    }

    public override async Task<MessageItem> CreateMessage(CreateMessageRequest request, ServerCallContext context)
    {
        
        var newMessage = new MessageItem
        {
            MessageId = Guid.NewGuid().ToString(),
            UserId = request.UserId,
            Content = request.Content,
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
        };

        await _broadcaster.BroadcastAsync(request.ChannelId, newMessage);
        await this.SaveMessageToDatabase(newMessage);
        return newMessage;
    }

    private async Task<bool> SaveMessageToDatabase(MessageItem message) 
    {
        Models.Message dbMessage = new() {
            UserId=message.UserId,
            Content=message.Content,
            Timestamp=DateTimeOffset.FromUnixTimeSeconds(message.Timestamp).UtcDateTime,
        };
        await this.messageDb.AddAsync(dbMessage);
        await this.messageDb.SaveChangesAsync();
        return true;
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
                
                logger.LogInformation("Pushed message {MessageId} to stream", newMessage.MessageId);
            }
        }
        catch (OperationCanceledException)
        {
            logger.LogInformation("Client disconnected from stream: {ChannelId}", request.ChannelId);
        }
    }
}
