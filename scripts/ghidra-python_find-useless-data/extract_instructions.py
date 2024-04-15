# Developed by : Nicolas Iooss
# All rights reserved

for block in currentProgram.getMemory().blocks:
    addr = block.start
    while addr != block.end:
        instr = currentProgram.listing.getInstructionAt(addr)
        if instr is None:
            addr = addr.add(1)
        else:
            print(addr, instr.length, instr)
            addr = addr.add(instr.length)