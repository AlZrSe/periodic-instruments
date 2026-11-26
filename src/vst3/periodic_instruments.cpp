// Periodic Instruments - Minimal VST3 Plugin

#include "public.sdk/source/vst/vstaudioeffect.h"
#include "public.sdk/source/main/pluginfactory.h"
#include "base/ustring.h"

using namespace Steinberg;
using namespace Steinberg::Vst;

static const FUID kProcessorUID (0x70657269, 0x6F646963, 0x496E7374, 0x72756D00);

class PeriodicInstrumentsProcessor : public AudioEffect
{
public:
    PeriodicInstrumentsProcessor () : AudioEffect ()
    {
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
        return kResultTrue;
    }
};

static FUnknown* createProcessor (void*) { return (IAudioProcessor*)new PeriodicInstrumentsProcessor (); }

extern "C" {

__attribute__((visibility("default"))) __attribute__((used)) IPluginFactory* GetPluginFactory ()
{
    static PFactoryInfo factoryInfo ("AlZrSe",
        "https://github.com/AlZrSe/periodic-instruments",
        "",
        Vst::kDefaultFactoryFlags);

    static CPluginFactory factory (factoryInfo);

    static bool once = true;
    if (once)
    {
        once = false;

        static PClassInfo processorInfo (kProcessorUID,
            PClassInfo::kManyInstances,
            kVstAudioEffectClass,
            "Periodic Instruments");
        factory.registerClass (&processorInfo, createProcessor);
    }

    return &factory;
}

} // extern "C"
