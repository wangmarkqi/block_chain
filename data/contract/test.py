import json
def terms(args):
    res=args+"\n hello from terms"
    return res

def after():
    print ("this is test")

    # must return str no json dumps in return
    return "ok"

def rollback():
    return "ok"
