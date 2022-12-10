# -*- coding: utf-8 -*-
"""
Sweaty Heart 💦 💖
------------------
A small python library written in rust to calculate the average sweat and heart rates.
"""

from .sweaty_heart import SignalProcessor, SerialReader

__version__ = '0.1.0'
__author__ = 'Kevin Silvester'
__license__ = "MIT"
__copyright__ = "Copyright 2022 Kevin Silvester"

__all__ = ["SignalProcessor", "SerialReader"]
