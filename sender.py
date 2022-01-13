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
    "H", ")", "K", "/", "O", "!", "X", "§",

    "Q", "J", "%", "³", "G", "°", "Ü", "`",
    "Ö", ">", "Ä", "#", "t", "x", "q", "ß",

    "ü", "ö", "ä", "<", "k", "p", "h", "o",
    "g", "n", "r", "s", "e", "a", "i", "b",

    "u", "d", "p", "z",
] 

try:
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "Online"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "44ad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "45ad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "46ad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "47ad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "48ad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "Offline"}))
except Exception as e:
    print(e)