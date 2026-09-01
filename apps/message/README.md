# Message Service

---

## Getting Started

### 1. Prerequisites & Environment Setup

Make sure you are in the service directory (`apps/message`) and

- [.NET 8.0 SDK](https://dotnet.microsoft.com/download/dotnet/8.0) or higher
- [PostgreSQL](https://www.postgresql.org/) database instance / or in docker
- VS Code (with C# Dev Kit) or Visual Studio 2022 / JetBrains Rider

#### 2. Set-up environment variables

_create `.env` file or export hese vars in your shell environment_

_example_:

```
MESSAGE_SERVICE_URL=http://localhost:3001
MESSAGE_SQL_CONNECTION_STRING="Host=localhost;Port=5432;Database=messages_db;Username=postgres;Password=password"
```

### 3. Database Migrations

Apply EF Core migrations to ensure the PostgreSQL schema is up to date:

`dotnet ef database update`

(Optional) To generate a new migration after updating models:

`dotnet ef migrations add <MigrationName>`

### 4. Runing the Service

cli:
`dotnet watch run `
or
`dotnet run`
or
VisualStudioCode / VsCode:
`f5` to start debugging
`ctrl + f5` to run without debugger
