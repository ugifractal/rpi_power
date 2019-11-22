# rpi_power
script to autoshutdown

## Disable built in Raspberry pi 3B soundcard
edit /boot/config.txt

From
```
dtparam=audio=on
```

to

```
# dtparam=audio=on
```

## Install

```
git clone https://github.com/ugifractal/rpi_power.git
cd rpi_power
cargo build --release
sudo cp rpi_power.service /etc/systemd/system/rpi_power.service
sudo systemctl start rpi_power.service
sudo systemctl enable rpi_power.service
```




