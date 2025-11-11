import traceback
import logging

try:
    raise Exception("boom boom boom")
except:
    print(traceback.format_exc())

# assertions; can be disabled by running with -O
try:
    doors = 'closed'
    assert doors == 'open', 'the doors need to be open'
except:
    print("assertion failed!")
    
logging.basicConfig(level=logging.DEBUG, format=' %(asctime)s - %(levelname)s- %(message)s')
logging.debug("here we go, program started")