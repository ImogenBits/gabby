from time import sleep
import httpx

#print(httpx.post("http://192.168.178.151:80/SerialSend", data={"data": "Online"}))
#sleep(3)

try:
    print(httpx.post("http://192.168.178.25:80/SerialSend", data={"data": "Online"}))
except Exception as e:
    print(e)