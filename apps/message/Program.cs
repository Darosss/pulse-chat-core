
using dotenv.net;
using message.Services;

var builder = WebApplication.CreateBuilder(args);

DotEnv.Load(new(envFilePaths:["../../.env"])
);
 const string appUrlEnvKey = "MESSAGE_SERVICE_URL";
string? appUrl = Environment.GetEnvironmentVariable(appUrlEnvKey);


if (string.IsNullOrWhiteSpace(appUrl))
{
    throw new Exception($"{appUrlEnvKey} is not set in environment variables");
}

builder.Services.AddGrpc();
var app = builder.Build();

app.MapGrpcService<MessageServiceInternal>();

app.Run(appUrl.Trim());
