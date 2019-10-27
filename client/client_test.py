'''
* based on Identity Based Encryption (Boneh-Franklin)
* type:           encryption (identity-based)
* setting:        bilinear groups (asymmetric)
* code reference: none
:Authors:    Anonymous works
:Date:       10/2019
'''

import unittest
import datetime

from charm.toolbox.pairinggroup import PairingGroup
from client import Typex_client


debug = False

class Typex_clientTest(unittest.TestCase):
	def testTypex_client(self):

		start_0= datetime.datetime.now()

		groupObj = PairingGroup('MNT224', secparam=1024)
		typex = Typex_client(groupObj)
		## prepare
		sender_id = 'typex_sender@gmail.com'
		# register the smart contract and get proof
		#sender_proof = "{left:29723213},{right:01024091},{left:10569293'},{root:81544474}";  
		sender_keys = typex.gen_key(sender_id)

		end_0= datetime.datetime.now()     
		delta_0 = end_0 - start_0
		print("the key generation time:",delta_0.total_seconds(), "seconds")

		## 
		receiver_id = 'typex_receiver@gmail.com'
		#receiver_proof = "{left:36821332},{right:72747743},{left:14217989'},{root:08754395}";  
		receiver_keys = typex.gen_key(receiver_id)

		## this algorithm is executed by the sender
		sender_proof = typex.user_register(sender_id);
		
		start_1= datetime.datetime.now()
		m = b"hello typex"
		ciphertext = typex.encrypt(receiver_id, receiver_keys['pk'], sender_proof, m)
		#print("Successful Encryption!",ciphertext)
		end_1= datetime.datetime.now()     
		delta_1 = end_1 - start_1
		print("the encryption time:",delta_1.total_seconds(), "seconds")

		## this algorithm is executed by the receiver
		sender_proof = typex.update(sender_id);
		start_2= datetime.datetime.now()
		msg = typex.decrypt(receiver_keys['sk'], receiver_id, sender_proof, ciphertext)
		#print("Successful Decryption!",msg)
		end_2= datetime.datetime.now()     
		delta_2 = end_2 - start_2
		print("the decrypttion time:",delta_2.total_seconds(), "seconds")

		assert msg == m, "failed decrypt: \n%s\n%s" % (msg, m)
		if debug: print("Successful Decryption!!!")


