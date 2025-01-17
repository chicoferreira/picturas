from typing import Any

from pydantic import BaseModel

from .core.messages.result_message import ResultMessage
from .bg_remover_request_message import BgRemoverRequestMessage


class BgRemoverResultOutput(BaseModel):
    type: str
    imageURI: str


class BgRemoverResultMessage(ResultMessage[BgRemoverResultOutput]):

    def __init__(self, request: BgRemoverRequestMessage, tool_result: Any, exception: Exception, *args):
        super().__init__(request, tool_result, exception, *args)
        if exception is None:
            self.output = BgRemoverResultOutput(
                type="image",
                imageURI=request.parameters.outputImageURI,
            )
