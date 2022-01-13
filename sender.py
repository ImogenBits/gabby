from time import sleep
import httpx

letter_map = [
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

letter_set = set(letter_map)
print(len(letter_map))
print(len(letter_set))
seen = set()
dupes = []

for x in letter_map:
    if x in seen:
        dupes.append(x)
    else:
        seen.add(x)
print(dupes)

try:
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"type": "control", "data": "online"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"type": "commands", "data": "58ad63ad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"type": "control", "data": "offline"}))
except Exception as e:
    print(e)