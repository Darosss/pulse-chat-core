using dotenv.net;
using message.Data;
using message.Errors;
using message.Services;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

DotEnv.Load(new(envFilePaths: ["../../.env"]));
const string appUrlEnvKey = "MESSAGE_SERVICE_URL";
string? appUrl = Environment.GetEnvironmentVariable(appUrlEnvKey);

const string dbConnectionKey = "MESSAGE_SQL_CONNECTION_STRING";
string? dbConnectionString = Environment.GetEnvironmentVariable(dbConnectionKey);
if (string.IsNullOrWhiteSpace(dbConnectionString))
{
    throw new Exception($"{dbConnectionKey} is not set in environment variables");
}
if (string.IsNullOrWhiteSpace(appUrl))
{
    throw new Exception($"{appUrlEnvKey} is not set in environment variables");
}

builder.Services.AddDbContext<MessageDbContext>(options =>
{
    options.UseNpgsql(dbConnectionString);
});
builder.Services.AddGrpc(options =>
{
    options.Interceptors.Add<ErrorHandlingInterceptor>();
});

builder.Services.AddSingleton<ChannelBroadcaster>();
var app = builder.Build();
using (var scope = app.Services.CreateScope())
{
    var dbContext = scope.ServiceProvider.GetRequiredService<MessageDbContext>();

    dbContext.Database.Migrate();
}
app.MapGrpcService<MessageServiceInternal>();
app.Run(appUrl.Trim());
