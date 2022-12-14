import serial
from sweaty_heart import SerialReader, SignalProcessor


class TestSignalProcessor:
    def test_signal_processor_init(self, benchmark):
        reader = SerialReader("/dev/ttyACM0", 115_200, 1000)
        proc = SignalProcessor(500, 3.5, 0.5)
        arduino = serial.Serial(port='/dev/ttyACM0',
                                baudrate=115200, timeout=.1)

        current_second = 0
        val_list = []

        while current_second < 1000:
            byte = arduino.readline()
            decoded_bytes = byte.decode("utf-8")

            if not decoded_bytes == "":
                data_str = decoded_bytes.strip()
                val_list.append(data_str)
                current_second += 1

        result = benchmark(proc.run, val_list)
