"""Main python script."""

import socket
from abc import abstractmethod
from collections.abc import Iterable
from dataclasses import dataclass, field
from enum import Enum
from typing import ClassVar, Final, Literal, Self


class Direction(Enum):
    """Directions the typewriter can move in."""

    # these values are fixed by the typewriter's encoding
    right = 0b00
    down = 0b01
    left = 0b10
    up = 0b11


type HorizontalDir = Literal[Direction.left, Direction.right, "No Movement"]


class Command:
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
    movement: HorizontalDir = Direction.right

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
            case "No Movement":
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
        assert self.distance is None or 1 <= self.distance < 2**8

    def encode(self) -> int:
        return 0x8300 | (self.distance or 0)


@dataclass
class Backspace(Command):
    """Moves the typewriter head a small amount to the left."""

    distance: int | None
    """The distance to be moved, or `None` to move the current character width."""

    def __post_init__(self) -> None:
        assert self.distance is None or 1 <= self.distance < 2**8

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


@dataclass
class Packet:
    is_response: bool
    data: bytes


class Connection:
    def __init__(self) -> None:
        self.received: list[bytes] = []
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    def start(self) -> None:
        self.sock.connect(("192.168.178.25:80", 80))

    def receive(self) -> Packet:
        received = self.sock.recv(1)[0]
        is_response = bool(received & 0x80)
        length = received & 0x7F
        data = self.sock.recv(length)
        return Packet(is_response, data)

    def send(self, data: int) -> bytes:
        self.sock.sendall(data.to_bytes(length=2, byteorder="big"))
        response = self.receive()
        while not response.is_response:
            self.received.append(response.data)
            response = self.receive()
        return response.data


@dataclass(kw_only=True)
class Typewriter:
    """High-level interface to the typewriter."""

    line_start: int = 0
    print_weight: int = 20
    line_height: int = 16
    feed_direction: HorizontalDir = Direction.right

    _horizontal_position: int = field(default=0, init=False)
    _vertical_position: int = field(default=0, init=False)
    _character_width: int = field(default=12, init=False)
    _connection: Connection = field(default=Connection(), init=False)

    def __post_init__(self) -> None:
        self._connection.start()

    @property
    def horizontal_position(self) -> int:
        return self._horizontal_position

    @property
    def vertical_position(self) -> int:
        return self.vertical_position

    @property
    def character_width(self) -> int:
        return self._character_width

    @character_width.setter
    def character_width(self, width: int) -> None:
        self._send(SetCharWidth(width))
        self._character_width = width

    def _send(self, data: Command | Iterable[Command]) -> list[bytes]:
        if isinstance(data, Command):
            data = [data]
        return [self._connection.send(command.encode()) for command in data]

    def online(self) -> None:
        self._send(Control.online())

    def offline(self) -> None:
        self._send(Control.offline())

    def __enter__(self) -> Self:
        self.online()
        return self

    def __exit__(self, *args: object) -> None:
        self.offline()

    def print_char(self, character: str, weight: int | None = None, feed: HorizontalDir | None = None) -> None:
        movement = feed or self.feed_direction
        self._send(PrintChar(character, weight or self.print_weight, movement))
        if movement != "No Movement":
            self._horizontal_position += (-1 if movement is Direction.left else 1) * self._character_width

    def newline(self) -> None:
        self._send(Move(Direction.down, self.line_height))
        self._vertical_position += self.line_height

    def carriage_return(self) -> None:
        assert self.horizontal_position >= self.line_start
        self._send(Move(Direction.left, self._horizontal_position - self.line_start))
        self._horizontal_position = self.line_start

    def space(self) -> None:
        self._send(Space(None))
        self._horizontal_position += self._character_width

    def print_string(self, data: str, weight: int | None = None, feed: HorizontalDir | None = None) -> None:
        for char in data:
            match char:
                case " ":
                    self.space()
                case "\n":
                    self.newline()
                    self.carriage_return()
                case _:
                    self.print_char(char, weight, feed)

    def move_head(self, horizontal: int, vertical: int) -> None:
        if horizontal:
            self._send(Move(Direction.left if horizontal < 0 else Direction.right, horizontal))
            self._horizontal_position += horizontal
        if vertical:
            self._send(Move(Direction.up if vertical < 0 else Direction.down, vertical))
            self._vertical_position += vertical

    def move_to(self, horizontal: int, vertical: int) -> None:
        self.move_head(horizontal - self._horizontal_position, vertical - self._vertical_position)
