from re import escape
from time import sleep
import httpx

def send(d):
    try:
        print(httpx.post("http://192.168.178.25:80/SerialSend", data=d))
    except Exception as e:
        print(e)

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
    "g", "n", "r", "s", "e", "a", "i", "b",

    "u", "d", "o", "z",
]
letters_map = {c: (i+1) for (i, c) in enumerate(letters)}
def commands(text):
    out = ""
    for c in text:
        out += f"{letters_map[c]:02x}ad"
    return out

cmd = commands("waah!")
send({"control": "on, off", "data": cmd})