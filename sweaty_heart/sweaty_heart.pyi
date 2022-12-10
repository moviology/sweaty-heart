from typing import Final, List, Optional


class SerialReader:
    """
    Reader object that will listen for data from specified serial port.
    """
    serial_port: Final[str]
    baud_rate: Final[int]
    timeout: Final[int]
    connection: Final[object]

    def __init__(self, serial_port: str, baud_rate: int,
                 timeout: int) -> None: ...

    def open(self) -> None:
        """Open connection to serial port"""
        ...

    def read(self) -> str:
        """Read the data from serial port and output as string"""
        ...

    def close(self) -> None:
        """Close serial port connection"""
        ...


class SignalProcessor:
    """
    Reader object that will listen for gsr and ppg sensor data on the specified
    serial port

    ...

    Attributes
    ----------
    serial_port : Final[str]
        The serial port the reader will be listening on

    lag : Final[int]
       The number of values before the current value that we wan tot use for computing the moving baseline

    threshold : Final[float]
        The number of standard deviations from the moving baseline that a peak needs to exceed to be counted

    influence : Final[float]
        the amount of influence that a peak has on the moving baseline. This must be between 0 and 1.

    ppg_signal: List[int]
        A vector holding the data from the ppg sensor

    gsr_signal: List[int]
        A vector holding the data from the gsr sensor

    ...

    Methods
    -------
    says(sound=None)
        Prints the animals name and what sound it makes
    """
    serial_port: Final[str]
    lag: Final[int]
    threshold: Final[float]
    influence: Final[float]
    ppg_signal: List[int]
    gsr_signal: List[int]

    def __init__(self, serial_port: str, lag: int,
                 threshold: float, influence: float) -> None: ...
