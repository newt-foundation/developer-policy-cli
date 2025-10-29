from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


@dataclass
class HttpRequest:
    url: str
    method: str
    headers: List[Tuple[str, str]]
    body: Optional[bytes]

@dataclass
class HttpResponse:
    status: int
    headers: List[Tuple[str, str]]
    body: bytes


def fetch(request: HttpRequest) -> HttpResponse:
    """
    Raises: `wit_world.types.Err(wit_world.imports.str)`
    """
    raise NotImplementedError

