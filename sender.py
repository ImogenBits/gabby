from time import sleep
import httpx


try:
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "Online"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "4aad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "4bad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "4cad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "4dad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "4ead"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "4fad"}))
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "Offline"}))
except Exception as e:
    print(e)