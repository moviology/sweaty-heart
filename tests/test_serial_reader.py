from sweaty_heart import SerialReader


# reader = None


# def init():
#     reader = SerialReader("/dev/ttyACM0", 115_200, 1)


# def test_init(benchmark):
#     benchmark(init)
# class TestSerialReader:
#     reader = None

#     def init(self):
#         reader = SerialReader("/dev/ttyACM0", 115_200, 20)

#     def test_init(self, benchmark):
#         benchmark(self.init)

#     def test_open(self, benchmark):
#         reader = SerialReader("/dev/ttyACM0", 115_200, 20)
#         reader.open()
#         val = reader.read()
#         print(val)
