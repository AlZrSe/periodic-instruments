// Periodic Instruments - VST3 Plugin
// A VST3 instrument plugin featuring 94 elemental voices (Hydrogen through Plutonium)

#include "public.sdk/source/vst/vstaudioeffect.h"
#include "public.sdk/source/vst/vsteditcontroller.h"
#include "public.sdk/source/main/pluginfactory.h"
#include "base/ustring.h"
#include "base/fstrdefs.h"

using namespace Steinberg;
using namespace Steinberg::Vst;

//------------------------------------------------------------------------
// Plugin IDs
//------------------------------------------------------------------------
static const FUID kProcessorUID (0x70657269, 0x6F646963, 0x496E7374, 0x72756D00);
static const FUID kControllerUID (0x70657269, 0x6F646963, 0x496E7374, 0x72430000);

//------------------------------------------------------------------------
// Audio Processor
//------------------------------------------------------------------------
class PeriodicInstrumentsProcessor : public AudioEffect
{
public:
    PeriodicInstrumentsProcessor () : AudioEffect ()
    {
        setControllerClass (kControllerUID);
    }

    tresult PLUGIN_API initialize (FUnknown* context) override
    {
        tresult result = AudioEffect::initialize (context);
        if (result == kResultTrue)
        {
            addAudioInput (STR16 ("Audio Input"), SpeakerArr::kStereo);
            addAudioOutput (STR16 ("Audio Output"), SpeakerArr::kStereo);
        }
        return result;
    }

    tresult PLUGIN_API process (ProcessData& data) override
    {
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

        info.title[0] = 0;
        info.shortTitle[0] = 0;
        info.units[0] = 0;

        for (int i = 0; titles[paramIndex][i] && i < 63; i++)
            info.title[i] = titles[paramIndex][i];
        for (int i = 0; shortTitles[paramIndex][i] && i < 63; i++)
            info.shortTitle[i] = shortTitles[paramIndex][i];

        info.id = paramIndex;
        info.stepCount = (paramIndex == 0) ? 93 : 0;
        info.defaultNormalizedValue = (paramIndex == 0) ? 0.0 : 0.5;
        info.unitId = 0;
        info.flags = ParameterInfo::kCanAutomate;
        if (paramIndex == 0)
            info.flags |= ParameterInfo::kIsList;

        return kResultTrue;
    }

    IPlugView* PLUGIN_API createView (const char* name) override
    {
        return nullptr;
    }
};

//------------------------------------------------------------------------
// Factory
//------------------------------------------------------------------------
static FUnknown* createProcessor (void*) { return (IAudioProcessor*)new PeriodicInstrumentsProcessor (); }
static FUnknown* createController (void*) { return (IEditController*)new PeriodicInstrumentsController (); }

extern "C" {

__attribute__((visibility("default"))) __attribute__((used)) IPluginFactory* GetPluginFactory ()
{
    if (!gPluginFactory)
    {
        static PFactoryInfo factoryInfo ("AlZrSe",
            "https://github.com/AlZrSe/periodic-instruments",
            "",
            Vst::kDefaultFactoryFlags);
        gPluginFactory = new CPluginFactory (factoryInfo);

        static PClassInfo processorInfo (kProcessorUID,
            PClassInfo::kManyInstances,
            kVstAudioEffectClass,
            "Periodic Instruments");
        gPluginFactory->registerClass (&processorInfo, createProcessor);

        static PClassInfo controllerInfo (kControllerUID,
            PClassInfo::kManyInstances,
            kVstComponentControllerClass,
            "Periodic Instruments Controller");
        gPluginFactory->registerClass (&controllerInfo, createController);
    }
    else
    {
        gPluginFactory->addRef ();
    }

    return gPluginFactory;
}

} // extern "C"
