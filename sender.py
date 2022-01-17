from re import escape
from time import sleep
import httpx

def send(d):
    a = d.upper().replace(" ", "")
    try:
        print(httpx.post("http://192.168.178.25:80/", data={"data": a}).content)
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
    send(cmdln(text))






send(cmdln("A" * (3000)))


"""void switch_online(void) {
    byte data[] = {0xA0, 0x00, 0xA1, 0x00, 0xA4, 0x00, 0xA2, 0x00};
    send_bytes(data, 8);
}

void switch_offline(void) {
    byte data[] = {0xA3, 0x00, 0xA0, 0x00};
    send_bytes(data, 4);
}"""