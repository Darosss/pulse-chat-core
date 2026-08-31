from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from collections.abc import Iterable as _Iterable, Mapping as _Mapping
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class CreateMessageRequest(_message.Message):
    __slots__ = ("channel_id", "content")
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    CONTENT_FIELD_NUMBER: _ClassVar[int]
    channel_id: int
    content: str
    def __init__(self, channel_id: _Optional[int] = ..., content: _Optional[str] = ...) -> None: ...

class CreateDirectMessageRequest(_message.Message):
    __slots__ = ("recipient_id", "content")
    RECIPIENT_ID_FIELD_NUMBER: _ClassVar[int]
    CONTENT_FIELD_NUMBER: _ClassVar[int]
    recipient_id: int
    content: str
    def __init__(self, recipient_id: _Optional[int] = ..., content: _Optional[str] = ...) -> None: ...

class HistoryRequest(_message.Message):
    __slots__ = ("channel_id", "limit", "page")
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    PAGE_FIELD_NUMBER: _ClassVar[int]
    channel_id: int
    limit: int
    page: int
    def __init__(self, channel_id: _Optional[int] = ..., limit: _Optional[int] = ..., page: _Optional[int] = ...) -> None: ...

class DirectHistoryRequest(_message.Message):
    __slots__ = ("recipient_id", "limit", "page")
    RECIPIENT_ID_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    PAGE_FIELD_NUMBER: _ClassVar[int]
    recipient_id: int
    limit: int
    page: int
    def __init__(self, recipient_id: _Optional[int] = ..., limit: _Optional[int] = ..., page: _Optional[int] = ...) -> None: ...

class MessageItem(_message.Message):
    __slots__ = ("id", "user_id", "channel_id", "content", "timestamp")
    ID_FIELD_NUMBER: _ClassVar[int]
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    CONTENT_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    id: int
    user_id: int
    channel_id: int
    content: str
    timestamp: int
    def __init__(self, id: _Optional[int] = ..., user_id: _Optional[int] = ..., channel_id: _Optional[int] = ..., content: _Optional[str] = ..., timestamp: _Optional[int] = ...) -> None: ...

class HistoryResponse(_message.Message):
    __slots__ = ("messages",)
    MESSAGES_FIELD_NUMBER: _ClassVar[int]
    messages: _containers.RepeatedCompositeFieldContainer[MessageItem]
    def __init__(self, messages: _Optional[_Iterable[_Union[MessageItem, _Mapping]]] = ...) -> None: ...

class StreamRequest(_message.Message):
    __slots__ = ("channel_id",)
    CHANNEL_ID_FIELD_NUMBER: _ClassVar[int]
    channel_id: int
    def __init__(self, channel_id: _Optional[int] = ...) -> None: ...
