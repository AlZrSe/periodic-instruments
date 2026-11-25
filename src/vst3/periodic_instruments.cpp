// Periodic Instruments - Minimal VST3 Plugin
// A VST3 instrument plugin featuring 94 elemental voices (Hydrogen through Plutonium)

#include "public.sdk/source/vst/vstaudioeffect.h"
#include "public.sdk/source/vst/vsteditcontroller.h"
#include "public.sdk/source/main/pluginfactory.h"
#include "base/ustring.h"
#include "base/source/fstring.h"

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
		// Instrument mode: output silence (foundation build)
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

	tresult PLUGIN_API getTailSize (int32& tailSize)
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
SMTG_EXPORT_SYMBOL Steinberg::IPluginFactory* PLUGIN_API GetPluginFactory ()
{
	static Steinberg::PFactoryInfo factoryInfo ("AlZrSe",
		"https://github.com/AlZrSe/periodic-instruments",
		"",
		Steinberg::Vst::kDefaultFactoryFlags);

	static Steinberg::CPluginFactory factory (factoryInfo);

	static bool once = true;
	if (once)
	{
		once = false;
		Steinberg::PClassInfo processorInfo (kPeriodicInstrumentsProcessorUID,
			Steinberg::PClassInfo::kManyInstances,
			kVstAudioEffectClass,
			"Periodic Instruments");
		factory.registerClass (&processorInfo, createProcessor);

		Steinberg::PClassInfo controllerInfo (kPeriodicInstrumentsControllerUID,
			Steinberg::PClassInfo::kManyInstances,
			kVstComponentControllerClass,
			"Periodic Instruments Controller");
		factory.registerClass (&controllerInfo, createController);
	}

	return &factory;
}

} // extern "C"