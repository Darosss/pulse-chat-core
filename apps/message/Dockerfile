FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build
WORKDIR /src

COPY *.sln .
COPY message.csproj ./

RUN dotnet restore message.csproj

COPY . .
RUN dotnet publish -c Release -o /app/publish

FROM mcr.microsoft.com/dotnet/aspnet:10.0
WORKDIR /app
COPY --from=build /app/publish .

EXPOSE 50052
ENTRYPOINT ["dotnet", "message.dll"]