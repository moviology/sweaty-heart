import heartpy as hp

data = hp.get_data('your_file.txt')
working_data, measures = hp.process(data, 100.0)

print(working_data, measures)
