"""Main python script."""

import socket
from abc import abstractmethod
from collections.abc import Iterable
from dataclasses import dataclass, field
from enum import Enum, IntEnum
from pathlib import Path
from typing import ClassVar, Final, Literal, Self, overload


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

    def encode_bytes(self) -> bytes:
        return (0x010000 | self.encode()).to_bytes(length=3, byteorder="big")


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


class DataType(IntEnum):
    response = 0x00
    misc = 0x01
    keyboard = 0x02

    @classmethod
    def parse(cls, value: int) -> tuple[DataType, int]:
        if value & 0x80:
            return DataType.response, value & 0x7F
        for member in DataType:
            if value & member.value:
                return member, DATA_LENGTH[member]
        raise RuntimeError


DATA_LENGTH = {
    DataType.misc: 1,
    DataType.keyboard: 4,
}


@dataclass
class Packet:
    type: DataType
    data: bytes


class Connection:
    def __init__(self) -> None:
        self.received: list[Packet] = []
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    def start(self) -> None:
        self.sock.connect(("192.168.178.25", 80))

    @overload
    def receive(self, *, blocking: Literal[True] = True) -> Packet: ...
    @overload
    def receive(self, *, blocking: Literal[False]) -> Packet | None: ...

    def receive(self, *, blocking: bool = True) -> Packet | None:
        self.sock.setblocking(blocking)
        try:
            received = self.sock.recv(1)[0]
        except OSError:
            return None
        data_type, length = DataType.parse(received)
        data = self.sock.recv(length)
        return Packet(data_type, data)

    def send(self, data: bytes) -> bytes:
        self.sock.sendall(data)
        response = self.receive()
        while response.type is DataType.misc:
            self.received.append(response)
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
        return [self._connection.send(command.encode_bytes()) for command in data]

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

    def get_keys(self) -> int:
        return int.from_bytes(self._connection.send(0x02.to_bytes()), "big")


def main() -> None:
    tp = Typewriter()
    print("yay")
    with tp:
        print("yay")
        key_map: dict[str, int] = {}
        while True:
            pressed = input("Which key is pressed? ")
            if pressed == "":
                break
            bits = tp.get_keys()
            key_map[pressed] = bits
            print(f"{pressed}: {bits:032b}")
        formatted = "\n".join(f"{key: >6} {bits: >8x} {bits:032b}" for key, bits in key_map.items())
        print(formatted)
        Path().joinpath("keys.txt").write_text(formatted)


if __name__ == "__main__":
    main()
