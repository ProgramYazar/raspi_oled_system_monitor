![rust oled display example](images/o1.jpeg)

# rust i2c oled display (128x64) for raspberry pies: zero,2,3,4..

You can use this program for system monitoring on raspberry pi or compatible dev boards (maybe: bananapi, nanopi..)
This program using linux hal layer. I use this program by cross compiling. If you compile on raspberry pi
comment these lines:

```
# [build]
# target = "arm-unknown-linux-gnueabihf"
```

i compiled like this on mac os x catalina:

```
cargo build --target arm-unknown-linux-gnueabihf --release
```

After compile you can upload with rsync or scp:

```
rsync -avz target/arm-unknown-linux-gnueabihf/release/raspi_oled_system_monitor pi@zero.local:~/
or
rsync -avz target/arm-unknown-linux-gnueabihf/release/raspi_oled_system_monitor pi@[your_device_ip]:~/
```

this program spent max %1 cpu and %.5 memory on raspberry pi zero. this means nothing.
