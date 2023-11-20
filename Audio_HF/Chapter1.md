# Chapter-1
## Introduction


By nature, a sound wave is a continuous signal.
- which means it contains an infinite number of signal values at a given time
- This is a problem for digital devices, which expect finite arrays
- For it to be able to be processed, stored, and transmitted by digital devices, the continuous sound wave needs to be converted into a series of discrete values, known as "digital representation"


There are different file formats for sound files like `.wav`, `.mp3`, etc.
- The formats mainly differ in how they compress the digital representation of the audio signal


Analog --> digital process
- Analog signal is first captured by a microphone
- Microphone then converts it into an electric signal
- This signal is then digitised by an analog-to-digital converter to get the digital representation through sampling


## Sampling and Sampling Rate
Sampling is the process of measuring the value of a continuous signal at fixed time steps.
- The resultant sampled waveform is discrete because it contains a finite number of signal values at uniform intervals


The sampling rate is also called `sampling frequency`.
- It is the number of samples taken in one second
- Measured in Hertz (Hz)
- Example: CD-quality audio has a sampling rate of 44100 Hz (44.1 kHz)