// Periodic Instruments - Minimal VST3 Plugin
// A VST3 instrument plugin featuring 94 elemental voices (Hydrogen through Plutonium)

#include "public.sdk/source/vst/vstaudioeffect.h"
#include "public.sdk/source/vst/vsteditcontroller.h"
#include "public.sdk/source/main/pluginfactory.h"

using namespace Steinberg;
using namespace Steinberg::Vst;

//------------------------------------------------------------------------
// Plugin IDs
//------------------------------------------------------------------------
static const FUID kPeriodicInstrumentsProcessorUID (0x70657269, 0x6F646963, 0x496E7374, 0x72756D00);
static const FUID kPeriodicInstrumentsControllerUID (0x70657269, 0x6F646963, 0x496E7374, 0x72430000);

//------------------------------------------------------------------------
// Audio Processor
//------------------------------------------------------------------------
class PeriodicInstrumentsProcessor : public AudioEffect
{
public:
	PeriodicInstrumentsProcessor () : AudioEffect ()
	{
		setControllerClass (kPeriodicInstrumentsControllerUID);
	}

	tresult PLUGIN_API initialize (FUnknown* context) override
	{
		tresult result = AudioEffect::initialize (context);
		if (result == kResultTrue)
		{
			addAudioInput (USTRING ("Audio Input"), SpeakerArr::kStereo);
			addAudioOutput (USTRING ("Audio Output"), SpeakerArr::kStereo);
		}
		return result;
	}

	tresult PLUGIN_API process (ProcessData& data) override
	{
		// Bypass handling
		if (data.processMode & ProcessMode::kBypass)
		{
			if (data.inputs && data.outputs)
			{
				for (int32 i = 0; i < data.numInputs; ++i)
				{
					if (data.inputs[i].numChannels == data.outputs[i].numChannels)
					{
						for (int32 ch = 0; ch < data.inputs[i].numChannels; ++ch)
						{
							if (data.inputs[i].channelBuffers32[ch] && data.outputs[i].channelBuffers32[ch])
							{
								memcpy (data.outputs[i].channelBuffers32[ch],
								        data.inputs[i].channelBuffers32[ch],
								        data.numSamples * sizeof (float));
							}
						}
					}
				}
			}
			return kResultTrue;
		}

		// Instrument mode: output silence
		if (data.outputs)
		{
			for (int32 i = 0; i < data.numOutputs; ++i)
			{
				for (int32 ch = 0; ch < data.outputs[i].numChannels; ++ch)
				{
					if (data.outputs[i].channelBuffers32[ch])
					{
						memset (data.outputs[i].channelBuffers32[ch], 0,
						        data.numSamples * sizeof (float));
					}
				}
			}
		}
		return kResultTrue;
	}

	tresult PLUGIN_API setupProcessing (ProcessSetup& setup) override
	{
		if (setup.maxSamplesPerBlock > 4096)
			return kResultFalse;
		if (setup.symbolicSampleSize != kSample32)
			return kResultFalse;
		return AudioEffect::setupProcessing (setup);
	}

	tresult PLUGIN_API canProcessSampleSize (int32 symbolicSampleSize) override
	{
		return (symbolicSampleSize == kSample32) ? kResultTrue : kResultFalse;
	}

	tresult PLUGIN_API getTailSize (int32& tailSize) override
	{
		tailSize = 0;
		return kResultTrue;
	}

	tresult PLUGIN_API setState (IBStream* state) override { return kResultTrue; }
	tresult PLUGIN_API getState (IBStream* state) override { return kResultTrue; }
};

//------------------------------------------------------------------------
// Controller
//------------------------------------------------------------------------
class PeriodicInstrumentsController : public EditController
{
public:
	tresult PLUGIN_API initialize (FUnknown* context) override
	{
		tresult result = EditController::initialize (context);
		if (result == kResultTrue)
		{
			parameters.addParameter (STR16 ("Element Selector"), nullptr, 0, 0.0,
			                        ParameterInfo::kCanAutomate | ParameterInfo::kIsList, 93);
			parameters.addParameter (STR16 ("Mass"), nullptr, 1, 0.5, ParameterInfo::kCanAutomate, 0);
			parameters.addParameter (STR16 ("Reactivity"), nullptr, 2, 0.5, ParameterInfo::kCanAutomate, 0);
			parameters.addParameter (STR16 ("State"), nullptr, 3, 0.5, ParameterInfo::kCanAutomate, 0);
			parameters.addParameter (STR16 ("Radiance"), nullptr, 4, 0.5, ParameterInfo::kCanAutomate, 0);
		}
		return result;
	}

	tresult PLUGIN_API getParameterInfo (int32 paramIndex, ParameterInfo& info) override
	{
		if (paramIndex >= 5)
			return kResultFalse;

		const char* titles[] = {"Element Selector", "Mass", "Reactivity", "State", "Radiance"};
		const char* shortTitles[] = {"Element", "Mass", "React", "State", "Rad"};
		int32 stepCounts[] = {93, 0, 0, 0, 0};
		int32 flags[] = {
			ParameterInfo::kCanAutomate | ParameterInfo::kIsList,
			ParameterInfo::kCanAutomate,
			ParameterInfo::kCanAutomate,
			ParameterInfo::kCanAutomate,
			ParameterInfo::kCanAutomate
		};

		String str (titles[paramIndex]);
		UString (info.title, 64).assign (str);
		String shortStr (shortTitles[paramIndex]);
		UString (info.shortTitle, 64).assign (shortStr);
		UString (info.units, 64).assign (USTRING (""));

		info.id = paramIndex;
		info.stepCount = stepCounts[paramIndex];
		info.defaultNormalizedValue = (paramIndex == 0) ? 0.0 : 0.5;
		info.unitId = 0;
		info.flags = flags[paramIndex];
		info.type = kParameterTypeFloat;

		return kResultTrue;
	}

	IPlugView* PLUGIN_API createView (const char* name) override
	{
		return nullptr; // No editor in foundation build
	}
};

//------------------------------------------------------------------------
// Factory
//------------------------------------------------------------------------
static Steinberg::FUnknown* createProcessor (void*)
{
	return (Steinberg::Vst::IAudioProcessor*)new PeriodicInstrumentsProcessor ();
}

static Steinberg::FUnknown* createController (void*)
{
	return (Steinberg::Vst::IEditController*)new PeriodicInstrumentsController ();
}

bool InitModule ()
{
	return true;
}

bool DeinitModule ()
{
	return true;
}

extern "C" {

//------------------------------------------------------------------------
// VST3 Factory
//------------------------------------------------------------------------
DEF_CLASS_IID (PeriodicInstrumentsProcessor)
DEF_CLASS_IID (PeriodicInstrumentsController)

SMTG_EXPORT_SYMBOL Steinberg::IPluginFactory* PLUGIN_API GetPluginFactory ()
{
	static Steinberg::PFactoryInfo factoryInfo =
	{
		"AlZrSe", "https://github.com/AlZrSe/periodic-instruments", ""
	};

	static Steinberg::CPluginFactory factory (factoryInfo);

	static bool once = true;
	if (once)
	{
		once = false;
		factory.registerClass (&kPeriodicInstrumentsProcessorUID, createProcessor, "Periodic Instruments", "Instrument|Synth", "AlZrSe", "0.1.0", "VST 3.7.0", 0);
		factory.registerClass (&kPeriodicInstrumentsControllerUID, createController, "Periodic Instruments Controller", nullptr, "AlZrSe", "0.1.0", "VST 3.7.0", 0);
	}

	return &factory;
}

} // extern "C"