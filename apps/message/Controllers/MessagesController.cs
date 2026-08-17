using Microsoft.AspNetCore.Mvc;

namespace message.Controllers;

[ApiController]
[Route("[controller]")]
public class MessagesController : ControllerBase
{

    [HttpGet(Name = "GetMessages")]
    public IEnumerable<string> Get()
    {
        return ["first message", "2nd message"];
    }
}
