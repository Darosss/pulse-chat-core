using Grpc.Core;
using Message;
using message.Data;
using message.Models.Dto;
using message.Utils;
using Microsoft.EntityFrameworkCore;

namespace message.Services;

public class MessageServiceInternal(
    ChannelBroadcaster broadcaster,
    MessageDbContext dbContext,
    RoomService roomService,
    ILogger<MessageServiceInternal> logger
) : MessageService.MessageServiceBase
{
    private readonly ChannelBroadcaster _broadcaster = broadcaster;
    private readonly MessageDbContext messageDb = dbContext;
    private readonly ILogger<MessageServiceInternal> _logger = logger;
    private readonly RoomService _roomService = roomService;

    public override async Task<HistoryResponse> GetChannelHistory(
        HistoryRequest request,
        ServerCallContext context
    )
    {
        var userId = GrpcHeaderUtils.GetUserId(context.RequestHeaders);
        if (!await this._roomService.IsUserIdAMemberOfRoom(userId, request.ChannelId))
        {
            throw new RpcException(
                new Status(StatusCode.PermissionDenied, "You do not have access to this room")
            );
        }
        var historyResponse = new HistoryResponse() { };

        int pageSize = (int)request.Limit;
        int skipCount = ((int)request.Page - 1) * pageSize;
        var dbMessages = await this
            .messageDb.Messages.Where(m => m.ChannelId == request.ChannelId)
            .OrderByDescending(m => m.Timestamp)
            .Skip(skipCount)
            .Take((int)request.Limit)
            .ToListAsync();

        var messageItems = dbMessages.Select(
            (m) =>
                new MessageItem()
                {
                    ChannelId = m.ChannelId,
                    UserId = m.UserId,
                    Content = m.Content,
                    Timestamp = m.Timestamp.Ticks,
                    Id = m.Id,
                }
        );
        historyResponse.Messages.Add(messageItems);

        return historyResponse;
    }

    public override async Task<GetPrivateRoomIdResponse> GetPrivateRoomId(
        GetPrivateRoomIdRequest request,
        ServerCallContext context
    )
    {
        var userId = GrpcHeaderUtils.GetUserId(context.RequestHeaders);
        var channelId = await this._roomService.GetOrCreateDirectRoomAsync(
            userId,
            request.RecipientId
        );

        return new() { ChannelId = channelId };
    }

    public override async Task<HistoryResponse> GetDirectMessageHistory(
        DirectHistoryRequest request,
        ServerCallContext context
    )
    {
        var userId = GrpcHeaderUtils.GetUserId(context.RequestHeaders);

        var historyResponse = new HistoryResponse() { };
        var channelId = await this._roomService.GetOrCreateDirectRoomAsync(
            userId,
            request.RecipientId
        );
        int pageSize = (int)request.Limit;
        int skipCount = ((int)request.Page - 1) * pageSize;

        var dbMessages = await this
            .messageDb.Messages.Where(m => m.ChannelId == channelId)
            .OrderByDescending(m => m.Timestamp)
            .Skip(skipCount)
            .Take((int)request.Limit)
            .ToListAsync();

        var messageItems = dbMessages.Select(
            (m) =>
                new MessageItem()
                {
                    ChannelId = m.ChannelId,
                    UserId = m.UserId,
                    Content = m.Content,
                    Timestamp = m.Timestamp.Ticks,
                    Id = m.Id,
                }
        );
        historyResponse.Messages.Add(messageItems);

        return historyResponse;
    }

    private async Task<Models.Message> SaveMessageToDatabase(MessageDatabaseDTO message, int userId)
    {
        Models.Message dbMessage = new()
        {
            ChannelId = message.ChannelId,
            UserId = userId,
            Content = message.Content,
            Timestamp = DateTime.UtcNow,
        };
        await this.messageDb.AddAsync(dbMessage);
        await this.messageDb.SaveChangesAsync();
        return dbMessage;
    }

    public override async Task<MessageItem> CreateMessage(
        CreateMessageRequest request,
        ServerCallContext context
    )
    {
        var userId = GrpcHeaderUtils.GetUserId(context.RequestHeaders);
        if (!await this._roomService.IsUserIdAMemberOfRoom(userId, request.ChannelId))
        {
            throw new RpcException(
                new Status(StatusCode.PermissionDenied, "You do not have access to this room")
            );
        }
        var newMessage = await this.SaveMessageToDatabase(
            new() { ChannelId = request.ChannelId, Content = request.Content },
            userId
        );

        var messageItem = new MessageItem
        {
            Id = newMessage.Id,
            ChannelId = request.ChannelId,
            UserId = userId,
            Content = request.Content,
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
        };

        await _broadcaster.BroadcastAsync(request.ChannelId, messageItem);
        return messageItem;
    }

    public override async Task<MessageItem> CreateDirectMessage(
        CreateDirectMessageRequest request,
        ServerCallContext context
    )
    {
        var userId = GrpcHeaderUtils.GetUserId(context.RequestHeaders);
        var channelId = await this._roomService.GetOrCreateDirectRoomAsync(
            userId,
            request.RecipientId
        );
        var newMessage = await this.SaveMessageToDatabase(
            new() { ChannelId = channelId, Content = request.Content },
            userId
        );

        var messageItem = new MessageItem
        {
            Id = newMessage.Id,
            ChannelId = channelId,
            UserId = userId,
            Content = request.Content,
            Timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
        };

        await _broadcaster.BroadcastAsync(channelId, messageItem);
        return messageItem;
    }

    public override async Task StreamLiveMessages(
        StreamRequest request,
        IServerStreamWriter<MessageItem> responseStream,
        ServerCallContext context
    )
    {
        var userId = GrpcHeaderUtils.GetUserId(context.RequestHeaders);
        if (!await this._roomService.IsUserIdAMemberOfRoom(userId, request.ChannelId))
        {
            throw new RpcException(
                new Status(StatusCode.PermissionDenied, "You do not have access to this room")
            );
        }
        logger.LogInformation(
            "Client connected to stream for channel: {ChannelId}",
            request.ChannelId
        );

        var reader = _broadcaster.Subscribe(request.ChannelId);
        var headers = new Metadata { { "status", "connected" } };
        await context.WriteResponseHeadersAsync(headers);
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
            logger.LogInformation(
                "Client disconnected from stream: {ChannelId}",
                request.ChannelId
            );
        }
    }
}
