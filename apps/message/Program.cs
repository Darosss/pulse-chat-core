
using dotenv.net;
using message.Data;
using message.Services;
using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

DotEnv.Load(new(envFilePaths:["../../.env"]));
const string appUrlEnvKey = "MESSAGE_SERVICE_URL";
string? appUrl = Environment.GetEnvironmentVariable(appUrlEnvKey);

const string dbConnectionKey = "SQL_CONNECTION_STRING";
string? dbConnectionString = Environment.GetEnvironmentVariable(dbConnectionKey);
  if (string.IsNullOrWhiteSpace(dbConnectionString))
    {
        throw new Exception($"{dbConnectionKey} is not set in environment variables");
    }
if (string.IsNullOrWhiteSpace(appUrl))
{
    throw new Exception($"{appUrlEnvKey} is not set in environment variables");
}

var isDevelopment = builder.Environment.IsDevelopment();
if (isDevelopment)
{
    builder.Services.AddDbContext<MessageDbContext>(options =>
        options.UseSqlite(dbConnectionString));
}
else
{
  
    builder.Services.AddDbContext<MessageDbContext>(options =>
        options.UseSqlServer(dbConnectionString));
}

builder.Services.AddGrpc();
builder.Services.AddSingleton<ChannelBroadcaster>();
var app = builder.Build();

app.MapGrpcService<MessageServiceInternal>();

app.Run(appUrl.Trim());
