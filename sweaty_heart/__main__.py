from sweaty_heart import SerialReader


reader = SerialReader("/dev/ttyACM0", 115_200, 10)
reader.open()
val = reader.read()
