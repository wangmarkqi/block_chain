import requests
import json
import time
import socket
import redis
import urllib3

urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)
HOST = '127.0.0.1'  # The server's hostname or IP address
PORT = 8787  # The port used by the server


def tcp():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(b'Hello, world')
        data = s.recv(1024)

    print('Received', repr(data))


def testredis():
    r = redis.StrictRedis(host='localhost', port=6379)
    p = r.pubsub()
    p.subscribe('fromrust')

    while True:
        message = p.get_message()  # Checks for message
        if message:
            msg = message['data']
            print(msg)
            r.publish("frompy", "heoolo")
            time.sleep(1)


# "rpc_append_entry"
def httpquery():
    l = ['rpc_peers_url', 'rpc_self_role', "rpc_latest_heartbeat", "rpc_self_check", 'rpc_leader_url', "rpc_max_block"]
    for i in l:
        url = f"https://localhost:8000/query/{i}/0"
        a = requests.get(url, verify=False)
        print("**********************")
        print(url)
        print(a.text)


def httptran():
    contract= dict(
        contract='append_entry',
        args="3453453",
    )
    url = "http://localhost:9000/transaction"
    a = requests.post(url, json=contract)
    print("**********************")
    print(url)
    print(a.text)


def smartcontract():
    contract = dict(
        contract='test',
        url="127.0.0.1:8001",
        sign=True,
        after=True,
        args="1234",
    )
    url = "http://localhost:9000/transaction"
    a = requests.post(url, json=contract)
    print (a.text)
    res1=json.loads(a.text)
    v=res1["voucher"]
    time.sleep(4)
    res2=query_contract(v)
    print (res2)

    return res1
def query_contract(voucher="76aa9cad-92f4-465d-9495-264197134ada"):
    url = f"http://localhost:9000/query/contract_status/{voucher}"
    res2 = requests.get(url)
    return res2.text

def query_all():
    url = f"http://localhost:9000/query/all_blocks/all"
    res2 = requests.get(url)
    return res2.text
# smartcontract()
# httptran()
t=query_all()
print (t)