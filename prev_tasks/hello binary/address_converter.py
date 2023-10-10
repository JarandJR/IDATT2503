import struct

address = 0x00000000000401192  # Replace this with your address
p64_address = struct.pack("<Q", address)

print(p64_address)
