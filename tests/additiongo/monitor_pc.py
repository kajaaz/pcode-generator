import gdb

# Global variable to store the last known value of the program counter
last_pc = None

def pc_changed(event):
    global last_pc
    # Get the current value of the program counter
    current_pc = gdb.parse_and_eval("$pc")

    if last_pc is not None and current_pc != last_pc:
        print(f"Program counter changed from {last_pc} to {current_pc}")
    last_pc = current_pc

# Register the event handler
gdb.events.stop.connect(pc_changed)

# Set a breakpoint at the main function start and configure GDB to stop there
gdb.execute("break main.main")
gdb.execute("run")

# Inform the user that the script is loaded and monitoring changes
print("PC monitoring script loaded. Monitoring changes to the program counter...")
