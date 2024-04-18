"""Generated by Sideko (sideko.dev)"""

from typing import TypeVar, Type, Union, Dict, Any
from pydantic import BaseModel, ValidationError
from httpx import Response

from canva_connect_api.errors import InvalidResponseBodyException

T = TypeVar("T", bound=Union[BaseModel, Dict[str, Any]])


def cast_response(response: Response, model: Type[T]) -> T:
    """
    Casts network data to either a Pydantic model or a simple dict based on the expected type.
    """
    try:
        # Check if the expected type is a Pydantic model
        class Caster(BaseModel):
            data: model

        return Caster(data=response.json()).data
    except Exception as e:
        raise InvalidResponseBodyException(
            response=response, expected_type=model
        ) from e