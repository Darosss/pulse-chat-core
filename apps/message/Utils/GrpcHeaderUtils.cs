namespace message.Utils;

using Grpc.Core;

public static class GrpcHeaderUtils
{
    public static int GetUserId(Metadata headers)
    {
        var userId = headers.GetValue("x-user-id");

        if (string.IsNullOrEmpty(userId) || !int.TryParse(userId, out int userIdAsNumber))
        {
            throw new RpcException(
                new Status(
                    StatusCode.Unauthenticated,
                    "Missing proper x-user-id header in Gateway request"
                )
            );
        }

        return userIdAsNumber;
    }
}
