from __future__ import annotations
from time import sleep
from xmlrpc.client import Boolean
from httpx import post, Response

#* Types
Commands = str

#* Constants
letters = [
    ".", ",", "-", "v", "l", "m", "j", "w",
    "²", "µ", "f", "^", ">", "´", "+", "1",

    "2", "3", "4", "5", "6", "7", "8", "9",
    "0", "E", "£", "B", "F", "P", "S", "Z",

    "V", "&", "Y", "A", "T", "L", "$", "R",
    "*", "C", '"', "D", "?", "N", "I", "U",

    ")", "W", "_", "=", ";", ":", "M", "'",
    "H", "(", "K", "/", "O", "!", "X", "§",

    "Q", "J", "%", "³", "G", "°", "Ü", "`",
    "Ö", "<", "Ä", "#", "t", "x", "q", "ß",

    "ü", "ö", "ä", "y", "k", "p", "h", "c",
    "g", "n", "r", "s", "e", "a", "i", "d",

    "u", "b", "o", "z",
]
letters_map = {c: (i+1) for (i, c) in enumerate(letters)}

online_cmd = "A0 00 A1 00 A4 00 A2 00"
offline_cmd = "A3 00 A0 00"

#* functions
def send(commands: Commands) -> Response | None:
    cmd = commands.upper().replace(" ", "")
    try:
        return post("http://192.168.178.25:80/", data={"data": cmd})
    except Exception as e:
        print(e)

def encode(text) -> Commands:
    out = ""
    for c in text:
        if c == " ":
            out += "8300"
        else:
            out += f"{letters_map[c]:02x}ad"
    return out

def parse_and_encode(text: str) -> Commands:
    lines = text.split("\n")
    cmd = ""
    for l in lines:
        cmd += encode(l)
        cmd += f"e{12*len(l):03x} d013"
    return cmd

def write(text: str, switch_on_off: Boolean=False):
    cmd = parse_and_encode(text)
    if switch_on_off:
        cmd = online_cmd + cmd + offline_cmd
    
    response = send(cmd)

    if response is not None and response.status_code != 200:
        print(f"Error {response.status_code}: {response.content}")

def writeln(text: str):
    write(text + "\n", True)