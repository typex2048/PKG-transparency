'''
* based on Identity Based Encryption (Boneh-Franklin)
* type:           encryption (identity-based)
* setting:        bilinear groups (asymmetric)
* code reference: https://github.com/JHUISI/charm/blob/dev/charm/schemes/ibenc/ibenc_bf01.py
:Authors:    Anonymous works
:Date:       10/2019
'''

from charm.toolbox.pairinggroup import ZR,G1,G2,pair
from charm.core.math.integer import randomBits,integer,bitsize
from charm.toolbox.hash_module import Hash,int2Bytes,integer
from charm.toolbox.IBEnc import IBEnc
from charm.toolbox.pairinggroup import PairingGroup
from collections import defaultdict

debug = False
class Typex_client(IBEnc):
    
    def __init__(self, groupObj):
        IBEnc.__init__(self)
        global group,h,P
        group = groupObj
        h = Hash(group)
        P = group.random(G2)

    def gen_key(self,id):
        # Choose a random generator P of G1
        s = group.random(ZR)
        # Pick a random s ∈ Z^∗_q and set Q = sP.
        Q = s * P
        keys = {'pk': Q, 'sk':s}
        return keys

    def user_register(self, id):
        ## register the cureent user to the smart smart contract
        ## this is a simulation here
        return  self.smart_conrtact_simulation(id)
    
    def encrypt(self, rid, pk, proof, M): 
        # handle the message
        enc_M = self.encodeToZn(M)
        if bitsize(enc_M) / 8 > group.messageSize():
            print("Message cannot be encoded.")
            return None 
        Q_id = group.hash(proof, G1) #standard
        g_id = pair(Q_id, pk) # this is a pair 
        # Pick a random s ∈ Z^∗_q
        r = group.random(ZR)
        U, V = r * P, enc_M ^ h.hashToZn(g_id ** r)
        C = { 'U':U, 'V':V, 'RID':rid}
        return C

    def update(self, id):
        ## query the smart smart contract
        ## this is a simulation here
        return  self.smart_conrtact_simulation(id)

    def smart_conrtact_simulation(self, id):
        proofs = defaultdict(list)
        proofs.update({'typex_sender@gmail.com':'{left:29723213},{right:01024091},{left:10569293},{root:81544474}'})
        proofs.update({'typex_receiver@gmail.com':'{right:36821332},{right:72747743},{right:14217989},{root:08754395}'})
        return  proofs.get(id)
    
    def decrypt(self, sk, r_id, proof, ct):
        U, V, RID = ct['U'], ct['V'], ct['RID']

        if r_id != RID:
            print("Decryption Failed!!!")
            return None
        
        # update the proof based on SID
        sk_temp = sk * group.hash(proof, G1)
        dec_M = V ^ h.hashToZn(pair(sk_temp, U))
        M = self.decodeFromZn(dec_M)
        return M

    def encodeToZn(self, message):
        assert type(message) == bytes, "Input must be of type bytes"
        return integer(message)
        
    def decodeFromZn(self, element):
        if type(element) == integer:
            msg = int2Bytes(element)
            return msg
        return None

def main():
    groupObj = PairingGroup('MNT224', secparam=1024)
    typex = Typex_client(groupObj)
    #(pk, sk) = ibe.setup()
    
    ## prepare
    sender_id = 'typex_sender@gmail.com'
    # register the smart contract and get proof
    #sender_proof = "{left:29723213},{right:01024091},{left:10569293'},{root:81544474}";  
    sender_keys = typex.gen_key(sender_id)

    ## 
    receiver_id = 'typex_receiver@gmail.com'
    #receiver_proof = "{left:36821332},{right:72747743},{left:14217989'},{root:08754395}";  
    receiver_keys = typex.gen_key(receiver_id)

    ## this algorithm is executed by the sender
    sender_proof = typex.user_register(sender_id);
    m = b"hello typex"
    ciphertext = typex.encrypt(receiver_id, receiver_keys['pk'], sender_proof, m)
    print("Successful Encryption!",ciphertext)

    ## this algorithm is executed by the receiver
    sender_proof = typex.update(sender_id);
    msg = typex.decrypt(receiver_keys['sk'], receiver_id, sender_proof, ciphertext)
    print("Successful Decryption!",msg)
    
if __name__ == '__main__':
    main()   
