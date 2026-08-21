import hashlib
def verify_audit_trail(chain):
for i in range(1, len(chain)):
current = chain[i]
previous = chain[i-1]
if current['hash'] != hashlib.sha256(str(current['data']).encode()).hexdigest():
return False
if current['previous_hash'] != previous['hash']:
return False
return True
chain = [{'data': 'Genesis', 'hash': hashlib.sha256('Genesis'.encode()).hexdigest(), 'previous_hash': None}]
print("Audit valid? ", verify_audit_trail(chain))