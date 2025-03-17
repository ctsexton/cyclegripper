### CycleGripper

CLAP plugin port of Sam Pluta's SuperCollider UGen, "CycleGripper".

Engaging the CycleGripper will sample a random length slice of incoming audio and repeat it, swapping the left and right channels on each repeat. Jarring!

Thank you to Sam Pluta for the original idea, a module in his Live Modular Instrument.

#### How to use:
1. Load audio plugin in a CLAP supporting audio host
2. Connect stereo inputs and outputs to PLUGIN
3. Connect a MIDI instrument to MIDI IN
4. Playing middle C (Note 60) will engage the cyclegripper effect
5. Playing any other note will disengage the cyclegripper effect
