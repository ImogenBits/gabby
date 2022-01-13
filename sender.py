from re import escape
from time import sleep
import httpx

def send(d, c=""):
    a = d.upper().replace(" ", "")
    print(a)
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
    "g", "n", "r", "s", "e", "a", "i", "b",

    "u", "d", "o", "z",
]
letters_map = {c: (i+1) for (i, c) in enumerate(letters)}
def commands(text):
    out = ""
    for c in text:
        out += f"{letters_map[c]:02x}ad"
    return out

for k in range(10):
    cmd = ""
    for i in range(k, k+10):
        cmd += f"{i:02x}ad"
    

#send("", "onoffmagic")

send("08AD E00C D008 08AD", "onoff")

#send("", "on")
#send("08AD")
#send("E009")
#send("D008")
#send("08AD")
#send("", "off")