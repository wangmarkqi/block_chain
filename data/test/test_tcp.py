import requests
import json
import socket  #导入模块
from multiprocessing import Pool
def test_tcp(j):
    print (j)
    for i in range(100):
        s=socket.socket(socket.AF_INET,socket.SOCK_STREAM)
        s.connect(('127.0.0.1',8000))
        l="adfa"*10000000
        s.send(l.encode())
        res = s.recv(1024).decode()
        s.close()
        print (res)

if __name__ == '__main__':
    l=list(range(1000))
    with Pool() as p:
        p.map(test_tcp,l)
