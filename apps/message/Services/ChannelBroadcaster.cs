using System.Collections.Concurrent;
using System.Threading.Channels;
using Message;

namespace message.Services;

public class ChannelBroadcaster
{
    private readonly ConcurrentDictionary<int, ConcurrentBag<ChannelWriter<MessageItem>>> _subscribers = new();

public ChannelReader<MessageItem> Subscribe(int channelId)
    {
        var channel = Channel.CreateUnbounded<MessageItem>(new UnboundedChannelOptions
        {
            SingleReader =true, SingleWriter = false
        });
        var writers = _subscribers.GetOrAdd(channelId, _ => []);
        writers.Add(channel.Writer);
        return channel.Reader;
    }
    public async Task BroadcastAsync(int channelId, MessageItem message)
    {
        if (_subscribers.TryGetValue(channelId, out var writers))
        {
            foreach (var writer in writers)
            {
                await writer.WriteAsync(message);
            }
        }
    }
}