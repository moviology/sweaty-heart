import serial
import time
import datetime
from sweaty_heart import SerialReader, SignalProcessor


class ReadLine:
    def __init__(self, s):
        self.buf = bytearray()
        self.s = s

    def readline(self):
        i = self.buf.find(b"\n")
        if i >= 0:
            r = self.buf[:i+1]
            self.buf = self.buf[i+1:]
            return r
        while True:
            i = max(1, min(2048, self.s.in_waiting))
            data = self.s.read(i)
            i = data.find(b"\n")
            if i >= 0:
                r = self.buf + data[:i+1]
                self.buf[0:] = data[i+1:]
                return r
            else:
                self.buf.extend(data)


# reader = SerialReader("/dev/ttyACM0", 115_200, 1000)
proc = SignalProcessor(3, 3.5, 0.5)

arduino = serial.Serial(port='/dev/ttyACM0', baudrate=115_200, timeout=.1)
rl = ReadLine(arduino)

current_second = 0
val_list = []

start = datetime.datetime.now() + datetime.timedelta(seconds=5)

while datetime.datetime.now() < start:
    decoded_bytes = rl.readline().decode("utf-8")
#     byte = arduino.readline()
#     decoded_bytes = byte.decode("utf-8")

    if not decoded_bytes == "":
        data_str = decoded_bytes.strip()
        val_list.append(data_str)

# print("running proc")
y = proc.run(val_list)

with open('your_file.txt', 'w') as f:
    for line in y:
        f.write(f"{line}\n")
