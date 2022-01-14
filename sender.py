from re import escape
from time import sleep
import httpx

def send(d, c=""):
    a = d.upper().replace(" ", "")
    try:
        httpx.post("http://192.168.178.25:80/SerialSend",
            data={"control": c, "data": a})
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
    "g", "n", "r", "s", "e", "a", "i", "d",

    "u", "b", "o", "z",
]
letters_map = {c: (i+1) for (i, c) in enumerate(letters)}

def commands(text):
    out = ""
    for c in text:
        if c == " ":
            out += "8300"
        else:
            out += f"{letters_map[c]:02x}ad"
    return out

def cmdln(text: str) -> str:
    lines = text.split("\n")
    cmd = ""
    for l in lines:
        cmd += commands(l)
        cmd += f"e{12*len(l):03x} d013"
    return cmd


def typeln(text: str):
    send(cmdln(text), "onoff")






sleep(20)
send("", "on")
send(cmdln("We're no strangers to love"))
send(cmdln("You know the rules and so do I"))
send(cmdln("A full commitment's what I'm thinking of"))
send(cmdln("You wouldn't get this from any other guy"))
send("", "off")

#send("", "on")
#send(commands("wah"))
#send("", "off")