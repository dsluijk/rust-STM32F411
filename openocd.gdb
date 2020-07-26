target extended-remote :3333

monitor reset halt
set print asm-demangle on
set backtrace limit 32

break DefaultHandler
break HardFault
break rust_begin_unwind
break main

monitor arm semihosting enable

# # send captured ITM to the file itm.fifo
# # (the microcontroller SWO pin must be connected to the programmer SWO pin)
# # 8000000 must match the core clock frequency
monitor tpiu config internal itm.txt uart off 16000000

# # OR: make the microcontroller SWO pin output compatible with UART (8N1)
# # 8000000 must match the core clock frequency
# # 2000000 is the frequency of the SWO pin
# monitor tpiu config external uart off 16000000 1800000

# # enable ITM port 0
monitor itm port 0 on

load
stepi
