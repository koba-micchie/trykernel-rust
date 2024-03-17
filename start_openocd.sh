#!/bin/sh

openocd -s /usr/share/openocd/scripts -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000" 
# -c "program target/thumbv6m-none-eabi/debug/trykernel verify reset exit"

