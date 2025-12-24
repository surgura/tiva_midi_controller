## Windows flashing setup (TM4C123GXL)

1. Download **xPack OpenOCD (Windows)**  
   https://github.com/xpack-dev-tools/openocd-xpack/releases
2. Extract, e.g. `C:\xpack-openocd-0.12.0-7\`
3. Install **WinUSB** on **In-Circuit Debug Interface (Interface 2)** using Zadig  
   (do not change Interface 3)
4. Set in Makefile:  
   `OPENOCD = C:/xpack-openocd-0.12.0-7/bin/openocd.exe`
5. Plug in the LaunchPad
6. Run:  
   `make`  
   `make flash`

Notes: ICDI is **JTAG-only**. SWD and WSL are not supported.
