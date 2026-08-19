using Grpc.Core;
using Message;

namespace message.Services;

public class MessageServiceInternal(ILogger<MessageServiceInternal> logger) : MessageService.MessageServiceBase
{
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
    public override Task StreamLiveMessages(StreamRequest request, IServerStreamWriter<MessageItem> responseStream, ServerCallContext context)
    {
        return Task.FromResult("TODO: add StreamLiveMessages");
    }
}
