"""Main python script."""

from abc import abstractmethod
from dataclasses import dataclass
from enum import Enum
from typing import ClassVar, Final, Literal, Protocol, Self


class Direction(Enum):
    """Directions the typewriter can move in."""

    # these values are fixed by the typewriter's encoding
    right = 0b00
    down = 0b01
    left = 0b10
    up = 0b11


type HorizontalDir = Literal[Direction.left, Direction.right]


class Command(Protocol):
    """Parent for all commands the typewriter can receive."""

    @abstractmethod
    def encode(self) -> int: ...


@dataclass
class Move(Command):
    """Moves the typewriter head."""

    direction: Direction
    distance: int

    def __post_init__(self) -> None:
        assert 0 <= self.distance < 2**12

    @classmethod
    def newline(cls) -> Self:
        return cls(Direction.down, 16)

    def encode(self) -> int:
        return 0xC000 | (self.direction.value << 12) | self.distance


@dataclass
class ToHome(Command):
    """Resets a part or all of the typewriter to its default state."""

    carriage: bool
    color_tape: bool
    type_wheel: bool

    def encode(self) -> int:
        return 0x8200 | (self.carriage << 8) | (self.color_tape << 9) | (self.type_wheel << 10)


@dataclass
class SetCharWidth(Command):
    """Sets the length of automatic movement after each character is typed."""

    width: int

    def __post_init__(self) -> None:
        assert 0 <= self.width < 2**8

    def encode(self) -> int:
        return 0x8000 | self.width


@dataclass
class PrintChar(Command):
    """Prints a character on the typewriter."""

    character: str
    thickness: int = 42
    movement: HorizontalDir | None = Direction.right

    def __post_init__(self) -> None:
        assert 0 <= self.thickness < 2**6
        assert self.character in self._char_map

    _char_map: ClassVar[Final[dict[str, int]]] = {
        c: i + 1
        for i, c in enumerate(
            ".,-vlmjw²µf^>´+1234567890E£BFPSZV&YATL$R*C'D?NIU)W_=;:M'H(K/O!X§QJ%³G°Ü`Ö<Ä#txqßüöäykphcgnrseaiduboz"
        )
    }

    def encode(self) -> int:
        match self.movement:
            case None:
                movement = 0b00
            case Direction.right:
                movement = 0b10
            case Direction.left:
                movement = 0b11
        return (self._char_map[self.character] << 8) | (movement << 6) | self.thickness


@dataclass
class Space(Command):
    """Moves the typewriter head a small amount to the right."""

    distance: int | None
    """The distance to be moved, or `None` to move the current character width."""

    def __post_init__(self) -> None:
        assert self.distance is None or 1 <= self.distance < 2 ** 8

    def encode(self) -> int:
        return 0x8300 | (self.distance or 0)


@dataclass
class Backspace(Command):
    """Moves the typewriter head a small amount to the left."""

    distance: int | None
    """The distance to be moved, or `None` to move the current character width."""

    def __post_init__(self) -> None:
        assert self.distance is None or 1 <= self.distance < 2 ** 8

    def encode(self) -> int:
        return 0x8400 | (self.distance or 0)


class Control(Command, Enum):
    """A control character specially interpreted by the typewriter."""

    CLEAR = 0xA000
    START = 0xA100
    STX = 0xA200
    ETX = 0xA300
    ENQ = 0xA400

    def encode(self) -> int:
        return self.value

    @classmethod
    def online(cls) -> list[Control]:
        return [Control.CLEAR, Control.START, Control.ENQ, Control.STX]

    @classmethod
    def offline(cls) -> list[Control]:
        return [Control.ETX, Control.CLEAR]

