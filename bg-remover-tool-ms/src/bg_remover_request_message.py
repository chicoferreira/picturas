from pydantic import BaseModel

from .core.messages.request_message import RequestMessage


class BgRemoverParameters(BaseModel):
    inputImageURI: str
    outputImageURI: str


BgRemoverRequestMessage = RequestMessage[BgRemoverParameters]
