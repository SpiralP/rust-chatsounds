searchState.loadedDescShard("rodio", 0, "Audio playback library.\nSee the <code>BackendSpecificError</code> docs for more information …\nError opening stream with OS. See cpal::BuildStreamError …\nAttempting to decode the audio failed.\nFailed to get the stream config for device the given …\nThe <code>Device</code> implementation associated with the platform’s …\nA device that is capable of audio input and/or output.\nThe <code>Devices</code> iterator associated with the platform’s …\nAn error that might occur while attempting to enumerate …\nA host’s device iterator yielding only <em>input</em> devices.\nThe output device was lost.\nCould not find any output device\nA host’s device iterator yielding only <em>output</em> devices.\n<code>cpal::Stream</code> container. Also see the more useful …\nMore flexible handle to a <code>OutputStream</code> that provides …\nAn error occurred while attempting to play a sound.\nCould not start playing the stream, see …\nRepresents a value of a single sample.\nHandle to a device that outputs sounds.\nA sink that allows changing the position of the source and …\nThe stream type created by <code>build_input_stream_raw</code> and …\nErrors that might occur when interfacing with audio output.\nThe iterator type yielding supported input stream formats.\nThe iterator type yielding supported output stream formats.\nDescribes a single supported stream configuration, …\nCould not list supported stream configs for device. Maybe …\nMultiplies the value of this sample by the given amount.\nAppends a sound to the queue of sounds to play.\nAppends a sound to the queue of sounds to play.\nReturns a reference to the underlying platform specific …\nReturns a reference to the underlying platform specific …\nReturns a mutable reference to the underlying platform …\nReturns a mutable reference to the underlying platform …\nA simple source of samples coming from a buffer.\nCreate an input stream.\nCreate a dynamically typed input stream.\nCreate an output stream.\nCreate a dynamically typed output stream.\nRemoves all currently loaded <code>Source</code>s from the <code>Sink</code>, and …\nRemoves all currently loaded <code>Source</code>s from the <code>SpatialSink</code> …\nDecodes samples from an audio file.\nThe default input stream format for the device.\nThe default output stream format for the device.\nDestroys the sink without stopping the sounds that are …\nDestroys the sink without stopping the sounds that are …\nMixer that plays multiple sounds at the same time.\nReturns true if this sink has no more sounds to play.\nReturns true if this sink has no more sounds to play.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the position of the sound that’s being played.\nReturns the position of the sound that’s being played.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the underlying platform specific implementation of …\nReturns the underlying platform specific implementation of …\nGets if a sink is paused\nGets if a sound is paused\nReturns the number of sounds currently in the queue.\nReturns the number of sounds currently in the queue.\nLinear interpolation between two samples.\nThe human-readable name of the device.\nBuilds a new <code>Sink</code>.\nPauses playback of this sink.\nPauses playback of this sink.\nResumes playback of a paused sink.\nResumes playback of a paused sound.\nPlays a sound once. Returns a <code>Sink</code> that can be used to …\nPlays a source with a device until it ends.\nQueue that plays sounds one after the other.\nCalls <code>saturating_add</code> on the sample.\nSets the position of the sound emitter in 3 dimensional …\nSets the position of the left ear in 3 dimensional space.\nSets the position of the right ear in 3 dimensional space.\nChanges the speed of the sound.\nChanges the speed of the sound.\nChanges the volume of the sound.\nChanges the volume of the sound.\nSkips to the next <code>Source</code> in the <code>Sink</code>\nSleeps the current thread until the sound ends.\nSleeps the current thread until the sound ends.\nSources of sound and various filters.\nChanges the play speed of the sound. Does not adjust the …\nChanges the play speed of the sound. Does not adjust the …\nA simple source of samples coming from a static buffer.\nStops the sink by emptying the queue.\nStops the sink by emptying the queue.\nAn iterator yielding formats that are supported by the …\nAn iterator yielding output stream formats that are …\nConverts the sample to an f32 value.\nReturn a new stream &amp; handle using the default output …\nReturns a new stream &amp; handle using the given output …\nReturns a new stream &amp; handle using the given device and …\nBuilds a new <code>Sink</code>, beginning playback on a stream.\nBuilds a new <code>SpatialSink</code>.\nAttempts to seek to a given position in the current source.\nAttempts to seek to a given position in the current source.\nGets the volume of the sound.\nThe value <code>1.0</code> is the “normal” volume (unfiltered …\nReturns the value corresponding to the absence of sound.\nA buffer of samples treated as a source.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nBuilds a new <code>SamplesBuffer</code>.\nThis jumps in memory till the sample for <code>pos</code>.\nThe stream contained malformed data and could not be …\nSource of audio samples from decoding a file.\nError that can happen when creating a decoder.\nAn IO error occurred while reading, writing, or seeking …\nA default or user-defined limit was reached while decoding …\nSource of audio samples from decoding a file that never …\nNo streams were found by the decoder\nThe demuxer or decoder needs to be reset before continuing.\nThe format of the data has not been recognized.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nBuilds a new decoder.\nBuilds a new looped decoder.\nBuilds a new decoder from mp3 data.\nBuilds a new decoder from vorbis data.\nBuilds a new decoder from wav data.\nSymphonia decoders types\nFormat reader failed to seek\nDecoding failed on multiple consecutive packets\nCould not get next packet while refining seek position\nDecoding failed retrying on the next packet failed\nError returned when the try_seek implementation of the …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThe output of the mixer. Implements <code>Source</code>.\nThe input of the mixer.\nAdds a new source to mix to the existing ones.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nBuilds a new mixer.\nThe input of the queue.\nThe output of the queue. Implements <code>Source</code>.\nAdds a new source to the end of the queue.\nAdds a new source to the end of the queue.\nRemoves all the sounds from the queue. Returns the number …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nBuilds a new queue. It consists of an input and an output.\nSets whether the queue stays alive if there’s no more …\nOnly seeks within the current source.\nFilter that modifies each sample by a given value.\nAutomatic Gain Control filter for maintaining consistent …\nThis applies an audio filter, it can be a high or low pass …\nIterator that at the same time extracts data from the …\nCombines channels in input into a single mono source, then …\nGenerate a sine wave with an instantaneous frequency that …\nMixes one sound fading out with another sound fading in …\nA source that delays the given source by a certain amount.\nWhen the inner source is empty this decrements a …\nAn empty source.\nAn empty source which executes a callback function\nFilter that modifies raises the volume from silence over a …\nFilter that modifies lowers the volume to silence over a …\nInternal type used by <code>from_factory</code>.\nA source that chains sources provided by an iterator.\nWaveform functions.\nThe hound (wav) decoder ran into an issue\nFilter that adds a linear gain ramp to the source over a …\nFilter that modifies each sample by a given value.\nOne of the underlying sources does not support seeking\nAny other error probably in a custom Source\nWraps a source and makes it pausable by calling …\nCalls a function on a source every time a period elapsed.\nA source that repeats the given source.\nWrap the input and lazily converts the samples it provides …\nA rising sawtooth wave.\nOccurs when try_seek fails because the underlying decoder …\nAn infinite source that produces one of a selection of …\nA sinusoidal waveform.\nAn infinite source that produces a sine.\nA source that skips specified duration of the given source …\nWrap the source in a skippable. It allows ending the …\nA source of samples.\nA simple spatial audio source. The underlying source is …\nFilter that modifies each sample by a given value.\nA square wave, rising edge at t=0.\nThis is the same as <code>Skippable</code> see its docs\nThe symphonia decoder ran into an issue\nA source that truncates the given source to a certain …\nTracks the elapsed duration since the start of the …\nA triangle waveform.\nAn iterator that reads from a <code>Source</code> and converts the …\nAn source that produces samples with value zero (silence). …\nAmplifies the sound by the given value.\nApplies automatic gain control to the sound.\nStores the source in a buffer in addition to returning it. …\nReturns the number of channels. Channels are always …\nConvenience function to create a new <code>Chirp</code> source.\nRemove any filter set.\nConverts the samples of this source to another type.\nReturns the number of samples before the current frame …\nDelays the sound by a certain duration.\nFades in the sound.\nFades out the sound.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBuilds a source that chains sources built from a factory.\nBuilds a source that chains sources provided by an …\nReturns the position of the underlying source relative to …\nApplies a high-pass filter to the source.\nApplies a high-pass filter to the source while allowing …\nReturns a mutable reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the innner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns a reference to the inner source.\nReturns the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nReturns a mutable reference to the inner source.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nReturns the inner source.\nApplies a linear gain ramp to the sound.\nApplies a low-pass filter to the source. <strong>Warning</strong>: Probably …\nApplies a low-pass filter to the source while allowing the …\nMixes this source with another one.\nCreate an empty source which executes a callback function. …\nWrap the input source and make it mono. Play that mono …\nWhen the inner source is empty the AtomicUsize passed in …\nAn empty source that immediately ends without ever …\nWrap the input and lazily converts the samples it provides …\nCreate a new <code>TestWaveform</code> object that generates an endless …\nThe frequency of the sine.\nBuilds a new <code>SpatialSink</code>, beginning playback on a stream.\nWrap a <code>Source</code> and lazily convert its samples to a specific …\nCreate a new source that never ends and produces total …\nCreate a new source that never ends and produces total …\nMakes the sound pausable.\nCalls the <code>access</code> closure on <code>Self</code> the first time the source …\nRepeats this source forever.\nAdds a basic reverb effect.\nReturns the rate at which the source should be played. In …\nEnable or disable AGC processing.\nModifies the amplification factor.\nModifies the speed factor.\nMake the truncated source end with a FadeOut. The fadeout …\nSets whether the filter applies.\nSets the position of the emitter and ears in the 3D world.\nSets the volume for a given channel number. Will panic if …\nSkips the current source\nImmediately skips a certain duration of this source.\nAdds a method <code>Skippable::skip</code> for skipping this source. …\nWill the source remain playing at its position before the …\nChanges the play speed of the sound. Does not adjust the …\nStops the sound.\nMakes the sound stoppable.\nMixes this sound fading out with another sound fading in …\nTakes a certain duration of this source and then stops.\nModifies this filter so that it becomes a high-pass filter\nSame as to_high_pass but allows the q value (bandwidth) to …\nModifies this filter so that it becomes a low-pass filter.\nSame as to_low_pass but allows the q value (bandwidth) to …\nReturns the total duration of this source, if known.\nStart tracking the elapsed duration since the start of the …\nCan not support seek, in the end state we lose the …\nAttempts to seek to a given position in the current source.\nPos is seen from the perspective of the api user.\nWill only attempt a seek if both underlying sources …\n<code>try_seek()</code> does nothing on the sine generator. If you need …\nThe source that did not support seek\nA buffer of samples treated as a source.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nBuilds a new <code>StaticSamplesBuffer</code>.")