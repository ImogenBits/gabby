from time import sleep
import httpx

#print(httpx.post("http://192.168.178.151:80/SerialSend", data={"data": "Online"}))
#sleep(3)

print(httpx.post("http://192.168.178.151:80/SerialSend", data={"data": "Online"}))